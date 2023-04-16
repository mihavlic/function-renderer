//! Converts the function AST into a sequence of operations and then glsl code.
//!
//! This module borrows the ideas from fidget <https://github.com/mkeeter/fidget/blob/169e18e75194e064515da883bb303bbe2031afdc/fidget/src/core/ssa/tape.rs>.
//! The tape is constructed by teaversing the ast depth-first and then pushing operation to the tape when returning from nodes. This ensures a topologicel order.
//! The operations are represented in ssa form, where we can deduplicate same expressions and delete unused ones. Known constant value is tracked to enable constant folding.
use std::{f32::consts::FRAC_PI_2, fmt::Display, hash::Hash};

use graph::{device::LazyDisplay, storage::DefaultAhashMap};

use super::{BinaryOperation, BuiltingVariable, Constant, TernaryOperation, UnaryOperation};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SsaIndex(u32);
impl SsaIndex {
    fn from_usize(value: usize) -> Self {
        Self(value.try_into().unwrap())
    }
}
impl Display for SsaIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "v{}", self.0)
    }
}

struct SsaIndexDerivative(SsaIndex);
impl Display for SsaIndexDerivative {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}d", self.0)
    }
}

#[derive(Clone, Copy)]
pub struct TotalF32(pub f32);
impl PartialEq for TotalF32 {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}
impl PartialOrd for TotalF32 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl Ord for TotalF32 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}
impl Eq for TotalF32 {}
impl Hash for TotalF32 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum SsaExpression {
    Ternary {
        op: TernaryOperation,
        a: SsaIndex,
        b: SsaIndex,
        c: SsaIndex,
    },
    Binary {
        op: BinaryOperation,
        left: SsaIndex,
        right: SsaIndex,
    },
    Unary {
        op: UnaryOperation,
        child: SsaIndex,
    },
    Constant(TotalF32),
    Builtin(BuiltingVariable),
}

/// A linear sequence of operations which evaluated the density function.
#[derive(Default)]
pub struct Tape {
    tape: Vec<(SsaExpression, bool)>,
    expressions: DefaultAhashMap<SsaExpression, SsaIndex>,
}

impl Tape {
    pub fn new() -> Self {
        Self::default()
    }
    /// Get the constant value, if it is known.
    fn get_constant_value(&self, index: SsaIndex) -> Option<f32> {
        match &self.tape[index.0 as usize].0 {
            SsaExpression::Constant(v) => Some(v.0),
            _ => None,
        }
    }
    /// Mark and SsaIndex as used, meaning that it will get emitted into glsl.
    pub fn mark_used(&mut self, index: SsaIndex) {
        self.tape[index.0 as usize].1 = true;
    }
    pub fn add_ast(&mut self, exppression: &super::Expression) -> SsaIndex {
        let index = self.process_ast_impl(exppression);
        self.mark_used(index);
        index
    }
    /// Appends an expression to the Tape
    fn process_ast_impl(&mut self, exppression: &super::Expression) -> SsaIndex {
        let ssa = match exppression {
            super::Expression::Ternary { op, a, b, c } => {
                let op = *op;

                let a = self.process_ast_impl(a);
                let b = self.process_ast_impl(b);
                let c = self.process_ast_impl(c);

                let a_value = self.get_constant_value(a);
                let b_value = self.get_constant_value(b);
                let c_value = self.get_constant_value(c);

                if let (Some(a), Some(b), Some(c)) = (a_value, b_value, c_value) {
                    let v = TernaryOperation::eval(op, a, b, c);
                    SsaExpression::Constant(TotalF32(v))
                } else {
                    match op {
                        TernaryOperation::Select => SsaExpression::Ternary { op, a, b, c },
                    }
                }
            }
            super::Expression::Binary { op, left, right } => {
                let mut op = *op;

                let mut left = self.process_ast_impl(left);
                let mut right = self.process_ast_impl(right);

                let l_value = self.get_constant_value(left);
                let r_value = self.get_constant_value(right);

                'make_expr: {
                    if let (Some(left), Some(right)) = (l_value, r_value) {
                        let v = BinaryOperation::eval(op, left, right);
                        SsaExpression::Constant(TotalF32(v))
                    } else {
                        // represent equivalent operations only one way to encourage deduplication
                        match op {
                            BinaryOperation::Exp => {
                                if let Some(v) = r_value {
                                    if v.abs() < f32::EPSILON {
                                        break 'make_expr SsaExpression::Constant(TotalF32(1.0));
                                    }
                                    if v.abs() - 1.0 < f32::EPSILON {
                                        return left;
                                    }
                                    if v.fract().abs() < f32::EPSILON {
                                        let count = v.round() as u32;
                                        if 1 < count && count <= 4 || count % 2 == 1 {
                                            let prev = left;
                                            for _ in 0..(count - 1) {
                                                self.mark_used(left);
                                                left = self.add_internal(SsaExpression::Binary {
                                                    op: BinaryOperation::Mul,
                                                    left,
                                                    right: prev,
                                                });
                                            }
                                            self.mark_used(left);
                                            return left;
                                        }
                                    }
                                }
                            }
                            BinaryOperation::Greater => {
                                op = BinaryOperation::Lower;
                                std::mem::swap(&mut left, &mut right);
                            }
                            BinaryOperation::GreaterEq => {
                                op = BinaryOperation::LowerEq;
                                std::mem::swap(&mut left, &mut right);
                            }
                            BinaryOperation::Add | BinaryOperation::Mul | BinaryOperation::Eq => {
                                if left > right {
                                    std::mem::swap(&mut left, &mut right);
                                }
                            }
                            _ => {}
                        }

                        self.mark_used(left);
                        self.mark_used(right);

                        SsaExpression::Binary {
                            op: op,
                            left,
                            right,
                        }
                    }
                }
            }
            super::Expression::Unary { op, child } => {
                let child = self.process_ast_impl(child);
                if let Some(value) = self.get_constant_value(child) {
                    let v = UnaryOperation::eval(*op, value);
                    SsaExpression::Constant(TotalF32(v))
                } else {
                    self.mark_used(child);
                    SsaExpression::Unary { op: *op, child }
                }
            }
            super::Expression::Builtin(b) => {
                return *self
                    .expressions
                    .get(&SsaExpression::Builtin(*b))
                    .expect("Builtins must be predefined throught add_extenal()");
            }
            super::Expression::Constant(c) => {
                let value = match c {
                    Constant::E => std::f32::consts::E,
                    Constant::Pi => std::f32::consts::PI,
                };
                SsaExpression::Constant(TotalF32(value))
            }
            super::Expression::Number(v) => SsaExpression::Constant(TotalF32(*v)),
        };
        self.add_internal(ssa)
    }
    /// Adds the expression to the tape or returns the index of a one if they are the same.
    ///
    /// The internal version adds expressions which are initially marked as unused
    fn add_internal(&mut self, expression: SsaExpression) -> SsaIndex {
        let Self { tape, expressions } = self;

        *expressions
            .entry(expression.clone())
            .or_insert_with(move || {
                let index = SsaIndex::from_usize(tape.len());
                tape.push((expression, false));
                index
            })
    }
    /// Adds the expression to the tape or returns the index of a one if they are the same.
    pub fn add(&mut self, expression: SsaExpression) -> SsaIndex {
        let Self { tape, expressions } = self;

        *expressions
            .entry(expression.clone())
            .or_insert_with(move || {
                let index = SsaIndex::from_usize(tape.len());
                // external expressions are conservatively always marked as used
                tape.push((expression, true));
                index
            })
    }
    /// Formats the tape as glsl code. Note that it is just a sequence of statements and doesn't include a function body.
    ///
    /// If `differentiate` is true, the code includes a automatic reverse differentiation to calculate the function gradient.
    pub fn write_glsl(&self, differentiate: bool) -> String {
        let mut out = String::new();
        self.write_glsl_into(&mut out, differentiate);
        out
    }
    /// Formats the tape as glsl code into a string. Note that it is just a sequence of statements and doesn't include a function body.
    ///
    /// If `differentiate` is true, the code includes a automatic reverse differentiation to calculate the function gradient.
    pub fn write_glsl_into(&self, out: &mut String, differentiate: bool) {
        for (i, (s, used)) in self.tape.iter().enumerate() {
            if *used == false {
                continue;
            }

            let o = SsaIndex::from_usize(i);

            use std::fmt::Write;
            let value = LazyDisplay(|f| match *s {
                SsaExpression::Ternary { op, a, b, c } => match op {
                    TernaryOperation::Select => write!(f, "({a} > 0.0) ? {b} : {c}"),
                },
                SsaExpression::Binary {
                    op,
                    left: a,
                    right: b,
                } => match op {
                    BinaryOperation::Sub => write!(f, "{a} - {b}"),
                    BinaryOperation::Add => write!(f, "{a} + {b}"),
                    BinaryOperation::Div => write!(f, "{a} / {b}"),
                    BinaryOperation::Mul => write!(f, "{a} * {b}"),
                    BinaryOperation::Exp => write!(f, "pow({a}, {b})"),
                    BinaryOperation::Greater => write!(f, "float({a} > {b})"),
                    BinaryOperation::Lower => write!(f, "float({a} < {b})"),
                    BinaryOperation::GreaterEq => write!(f, "float({a} >= {b})"),
                    BinaryOperation::LowerEq => write!(f, "float({a} <= {b})"),
                    BinaryOperation::Eq => write!(f, "float({a} == {b})"),
                    BinaryOperation::Min => write!(f, "min({a}, {b})"),
                    BinaryOperation::Max => write!(f, "max({a}, {b})"),
                },
                SsaExpression::Unary { op, child: a } => match op {
                    UnaryOperation::Neg => write!(f, "-{a}"),
                    UnaryOperation::Log => write!(f, "log({a}) / log(10.0)"),
                    UnaryOperation::Log2 => write!(f, "log2({a})"),
                    UnaryOperation::Ln => write!(f, "log({a})"),
                    UnaryOperation::Sqrt => write!(f, "sqrt({a})"),
                    UnaryOperation::Sin => write!(f, "sin({a})"),
                    UnaryOperation::Cos => write!(f, "cos({a})"),
                    UnaryOperation::Tan => write!(f, "tan({a})"),
                    UnaryOperation::Abs => write!(f, "abs({a})"),
                    UnaryOperation::CoTan => write!(f, "1.0 / tan({a})"),
                    UnaryOperation::ArcSin => write!(f, "asin({a})"),
                    UnaryOperation::ArcCos => write!(f, "acos({a})"),
                    UnaryOperation::ArcTan => write!(f, "atan({a})"),
                    UnaryOperation::ArcCotan => write!(f, "{FRAC_PI_2} - atan({a})"),
                },
                SsaExpression::Constant(v) => write!(f, "{}", v.0),
                SsaExpression::Builtin(_) => panic!(),
            });
            let differentiation = LazyDisplay(|f: &mut std::fmt::Formatter| {
                let dout = SsaIndexDerivative(o);
                match *s {
                    SsaExpression::Ternary { op, a, b, c } => {
                        let db = SsaIndexDerivative(b);
                        let dc = SsaIndexDerivative(c);
                        match op {
                            TernaryOperation::Select => {
                                write!(f, "vec3 {dout} = ({a} > 0.0) ? {db} : {dc}")
                            }
                        }
                    }
                    SsaExpression::Binary {
                        op,
                        left: a,
                        right: b,
                    } => {
                        let da = SsaIndexDerivative(a);
                        let db = SsaIndexDerivative(b);
                        match op {
                            BinaryOperation::Sub => write!(f, "vec3 {dout} = {da} - {db}"),
                            BinaryOperation::Add => write!(f, "vec3 {dout} = {da} + {db}"),
                            BinaryOperation::Div => write!(f, "float square_{b} = {b} * {b};\nvec3 {dout} = ({b} * {da} - {a} * {db}) / square_{b}"),
                            BinaryOperation::Mul => write!(f, "vec3 {dout} = {b} * {da} + {a} * {db}"),
                            BinaryOperation::Exp => {
                                if let Some(b) = self.get_constant_value(b) {
                                    let b_1 = b - 1.0;
                                    write!(f, "vec3 {dout} = {da} * {b} * pow({a}, {b_1})")
                                } else if let Some(a) = self.get_constant_value(a)  {
                                    write!(f, "vec3 {dout} = {db} * {o} * log({a})")
                                } else {
                                    write!(f, "vec3 {dout} = 0.0 /* TODO */")
                                }
                            },
                            // step functions like this don't really have a derivative
                            BinaryOperation::Greater |
                            BinaryOperation::Lower |
                            BinaryOperation::GreaterEq |
                            BinaryOperation::LowerEq |
                            BinaryOperation::Eq => write!(f, "vec3 {dout} = 0.0"),
                            BinaryOperation::Min => write!(f, "vec3 {dout};\nif ({a} < {b}) {{ {dout} = {da}; }} else {{ {dout} = {db}; }}"),
                            BinaryOperation::Max => write!(f, "vec3 {dout};\nif ({a} < {b}) {{ {dout} = {db}; }} else {{ {dout} = {da}; }}"),
                        }
                    }
                    SsaExpression::Unary { op, child: a } => {
                        let da = SsaIndexDerivative(a);
                        use std::f32::consts::{LN_10, LN_2};
                        match op {
                            UnaryOperation::Neg => write!(f, "vec3 {dout} = -{da}"),
                            UnaryOperation::Log => write!(f, "vec3 {dout} = {da} / ({a} * {LN_10})"),
                            UnaryOperation::Log2 => write!(f, "vec3 {dout} = {da} / ({a} * {LN_2})"),
                            UnaryOperation::Ln => write!(f, "vec3 {dout} = {da} / {a}"),
                            UnaryOperation::Sqrt => write!(f, "vec3 {dout} = {da} / (2.0 * {o})"),
                            UnaryOperation::Sin => write!(f, "vec3 {dout} = {da} * cos({a})"),
                            UnaryOperation::Cos => write!(f, "vec3 {dout} = -{da} * sin({a})"),
                            UnaryOperation::Tan => write!(f, "float cos_{a} = cos({a});\nvec3 {dout} = {da} / (cos_{a} * cos_{a})"),
                            UnaryOperation::CoTan => write!(f, "float sin_{a} = sin({a});\nvec3 {dout} = -{da} / (sin_{a} * sin_{a})"),
                            UnaryOperation::Abs => write!(f, "vec3 {dout};\nif ({a} < 0.0) {{ {dout} = -{da}; }} else {{ {dout} = {da}; }}"),
                            UnaryOperation::ArcSin => write!(f, "vec3 {dout} = {da} / sqrt(1.0 - {a}*{a})"),
                            UnaryOperation::ArcCos => write!(f, "vec3 {dout} = -{da} / sqrt(1.0 - {a}*{a})"),
                            UnaryOperation::ArcTan => write!(f, "vec3 {dout} = {da} / (1.0 + {a}*{a})"),
                            UnaryOperation::ArcCotan => write!(f, "vec3 {dout} = -{da} / (1.0 + {a}*{a})"),
                        }
                    }
                    SsaExpression::Constant(_) => write!(f, "vec3 {dout} = vec3(0.0)"),
                    SsaExpression::Builtin(b) => match b {
                        BuiltingVariable::X | BuiltingVariable::normalized_X => {
                            write!(f, "vec3 {dout} = vec3(1.0, 0.0, 0.0)")
                        }
                        BuiltingVariable::Y | BuiltingVariable::normalized_Y => {
                            write!(f, "vec3 {dout} = vec3(0.0, 1.0, 0.0)")
                        }
                        BuiltingVariable::Z | BuiltingVariable::normalized_Z => {
                            write!(f, "vec3 {dout} = vec3(0.0, 0.0, 1.0)")
                        }
                        BuiltingVariable::T => write!(f, "vec3 {dout} = vec3(0.0, 0.0, 0.0)"),
                    },
                }
            });
            if let SsaExpression::Builtin(_) = s {
                // provided by caller
            } else {
                writeln!(out, "float {o} = {value};").unwrap();
            }
            if differentiate {
                writeln!(out, "{differentiation};").unwrap();
            }
        }
    }
}
