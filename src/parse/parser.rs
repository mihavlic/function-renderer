use std::{collections::VecDeque, fmt::Display};

#[derive(Debug)]
pub struct ParserError {
    pub message: String,
}

macro_rules! parse_error {
    ($($a:tt)*) => {
        Err(ParserError {
            message: format!($($a)*),
        })
    };
}

pub type Result<T> = std::result::Result<T, ParserError>;

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for ParserError {}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum TernaryOperation {
    Select,
}

impl TernaryOperation {
    pub fn eval(self, a: f32, b: f32, c: f32) -> f32 {
        match self {
            TernaryOperation::Select => {
                if a > 0.0 {
                    a
                } else {
                    b
                }
            }
        }
    }
    fn try_function_str(str: &str) -> Option<Self> {
        match str {
            "select" => Some(Self::Select),
            _ => None,
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            TernaryOperation::Select => "select",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryOperation {
    Sub,
    Add,
    Div,
    Mul,
    Exp,
    Greater,
    Lower,
    GreaterEq,
    LowerEq,
    Eq,
    Min,
    Max,
}

impl BinaryOperation {
    pub fn eval(self, a: f32, b: f32) -> f32 {
        match self {
            BinaryOperation::Sub => a - b,
            BinaryOperation::Add => a + b,
            BinaryOperation::Div => a / b,
            BinaryOperation::Mul => a * b,
            BinaryOperation::Exp => a.powf(b),
            BinaryOperation::Greater => (a > b) as u32 as f32,
            BinaryOperation::Lower => (a < b) as u32 as f32,
            BinaryOperation::GreaterEq => (a < b) as u32 as f32,
            BinaryOperation::LowerEq => (a >= b) as u32 as f32,
            BinaryOperation::Eq => (a <= b) as u32 as f32,
            BinaryOperation::Min => a.max(b),
            BinaryOperation::Max => a.min(b),
        }
    }
    fn try_function_str(str: &str) -> Option<Self> {
        match str {
            "min" => Some(Self::Min),
            "max" => Some(Self::Max),
            _ => None,
        }
    }
    fn try_from_token(token: &Token) -> Option<(Self, u8)> {
        let (precedence, token) = match token {
            Token::Sub => (4, BinaryOperation::Sub),
            Token::Add => (4, BinaryOperation::Add),
            Token::Div => (3, BinaryOperation::Div),
            Token::Mul => (2, BinaryOperation::Mul),
            Token::Exp => (1, BinaryOperation::Exp),
            Token::Greater => (5, BinaryOperation::Greater),
            Token::Lower => (5, BinaryOperation::Lower),
            Token::GreaterEq => (5, BinaryOperation::GreaterEq),
            Token::LowerEq => (5, BinaryOperation::LowerEq),
            Token::Eq => (5, BinaryOperation::Eq),
            _ => return None,
        };

        Some((token, precedence))
    }
    pub fn name(self) -> &'static str {
        match self {
            BinaryOperation::Sub => "sub",
            BinaryOperation::Add => "add",
            BinaryOperation::Div => "div",
            BinaryOperation::Mul => "mul",
            BinaryOperation::Exp => "exp",
            BinaryOperation::Greater => "greater",
            BinaryOperation::Lower => "lower",
            BinaryOperation::GreaterEq => "greatereq",
            BinaryOperation::LowerEq => "lowereq",
            BinaryOperation::Eq => "eq",
            BinaryOperation::Min => "min",
            BinaryOperation::Max => "max",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryOperation {
    Neg,
    Log,
    Log2,
    Ln,
    Sqrt,
    Sin,
    Cos,
    Tan,
    Abs,
    CoTan,
    ArcSin,
    ArcCos,
    ArcTan,
    ArcCotan,
}

impl UnaryOperation {
    pub fn eval(self, a: f32) -> f32 {
        match self {
            UnaryOperation::Neg => -a,
            UnaryOperation::Log => a.log10(),
            UnaryOperation::Log2 => a.log2(),
            UnaryOperation::Ln => a.ln(),
            UnaryOperation::Sqrt => a.sqrt(),
            UnaryOperation::Sin => a.sin(),
            UnaryOperation::Cos => a.cos(),
            UnaryOperation::Tan => a.tan(),
            UnaryOperation::Abs => a.abs(),
            UnaryOperation::CoTan => a.tan().recip(),
            UnaryOperation::ArcSin => a.asin(),
            UnaryOperation::ArcCos => a.acos(),
            UnaryOperation::ArcTan => a.atan(),
            UnaryOperation::ArcCotan => std::f32::consts::FRAC_2_PI - a.atan(),
        }
    }
    fn try_function_str(str: &str) -> Option<Self> {
        match str {
            "arccos" => Some(Self::ArcCos),
            "arccotan" => Some(Self::ArcCotan),
            "arcsin" => Some(Self::ArcSin),
            "arctan" => Some(Self::ArcTan),
            "cos" => Some(Self::Cos),
            "cotan" => Some(Self::CoTan),
            "ln" => Some(Self::Ln),
            "log" => Some(Self::Log),
            "log2" => Some(Self::Log2),
            "sin" => Some(Self::Sin),
            "sqrt" => Some(Self::Sqrt),
            "tan" => Some(Self::Tan),
            "abs" => Some(Self::Abs),
            _ => None,
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            UnaryOperation::Neg => "neg",
            UnaryOperation::Log => "log",
            UnaryOperation::Log2 => "log2",
            UnaryOperation::Ln => "ln",
            UnaryOperation::Sqrt => "sqrt",
            UnaryOperation::Sin => "sin",
            UnaryOperation::Cos => "cos",
            UnaryOperation::Tan => "tan",
            UnaryOperation::Abs => "abs",
            UnaryOperation::CoTan => "cotan",
            UnaryOperation::ArcSin => "arcsin",
            UnaryOperation::ArcCos => "arccos",
            UnaryOperation::ArcTan => "arctan",
            UnaryOperation::ArcCotan => "arccotan",
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BuiltingVariable {
    X,
    Y,
    Z,
    T,
    X_normalized,
    Y_normalized,
    Z_normalized,
}

impl BuiltingVariable {
    pub fn try_from_str(str: &str) -> Option<Self> {
        match str {
            "X" | "x" => Some(Self::X),
            "Y" | "y" => Some(Self::Y),
            "Z" | "z" => Some(Self::Z),
            "t" => Some(Self::T),
            "x_normalized" => Some(Self::X_normalized),
            "y_normalized" => Some(Self::Y_normalized),
            "z_normalized" => Some(Self::Z_normalized),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum Constant {
    E,
    Pi,
}

impl Constant {
    pub fn try_from_str(str: &str) -> Option<Self> {
        match str {
            "e" => Some(Self::E),
            "pi" => Some(Self::Pi),
            _ => None,
        }
    }
}

pub enum Expression {
    Ternary {
        op: TernaryOperation,
        a: Box<Expression>,
        b: Box<Expression>,
        c: Box<Expression>,
    },
    Binary {
        op: BinaryOperation,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Unary {
        op: UnaryOperation,
        child: Box<Expression>,
    },
    Builtin(BuiltingVariable),
    Constant(Constant),
    Variable(String),
    Number(f32),
}

#[derive(Debug, Clone, Copy)]
enum Token<'a> {
    Sub,
    Add,
    Mul,
    Div,
    Abs,
    Exp,
    Num,
    Greater,
    Lower,
    GreaterEq,
    LowerEq,
    Eq,
    LParen,
    RParen,
    Comma,
    Ident(&'a str),
    Number(f32),
    End,
}

// "inspired" by https://github.com/gfx-rs/naga/blob/48e79388b506535d668df4f6c7be4e681812ab81/src/front/wgsl/lexer.rs#L8
fn next_token(mut str: &str) -> Result<(Token<'_>, &str)> {
    let mut chars = str.chars();
    loop {
        str = chars.as_str();
        let cur = match chars.next() {
            Some(c) => c,
            None => return Ok((Token::End, "")),
        };

        macro_rules! two_char_token {
            ($($char:pat => $token:expr),+ ; $single:expr) => {
                match chars.clone().next() {
                    $(
                        Some($char) => {
                            chars.next();
                            $token
                        },
                    )+
                        _  => $single
                }
            };
        }

        let tok = match cur {
            ',' => Token::Comma,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '*' => Token::Mul,
            '/' => Token::Div,
            '-' => Token::Sub,
            '+' => Token::Add,
            '=' => Token::Eq,
            '^' => Token::Exp,
            '|' => Token::Abs,
            '>' => two_char_token!(
                '=' => Token::GreaterEq
                ; Token::Greater
            ),
            '<' => two_char_token!(
                '=' => Token::LowerEq
                ; Token::Lower
            ),
            '0'..='9' => {
                let (number, next) = consume_number(str)?;
                return Ok((Token::Number(number), next));
            }
            _ if is_blankspace(cur) => continue,
            _ if is_identifier(cur) => {
                let (ident, rest) = consume_any(str, is_identifier);
                return Ok((Token::Ident(ident), rest));
            }
            _ => {
                return parse_error!("Unexpected character '{cur}'");
            }
        };

        return Ok((tok, chars.as_str()));
    }
}

fn consume_any(input: &str, what: impl Fn(char) -> bool) -> (&str, &str) {
    let pos = input.find(|c| !what(c)).unwrap_or(input.len());
    input.split_at(pos)
}

fn consume_number(str: &str) -> Result<(f32, &str)> {
    #[derive(PartialEq, Eq)]
    pub enum State {
        Start,
        Integer,
        Fractional,
    }

    let mut state = State::Start;
    let mut chars = str.char_indices();

    let mut dot_index = None;
    let mut end_index = None;

    while let Some((i, c)) = chars.next() {
        match c {
            '0'..='9' => {}
            '.' => {
                if state == State::Integer {
                    dot_index = Some(i);
                    state = State::Fractional;
                } else {
                    return parse_error!("Unexpected . when parsing number");
                }
            }
            _ => {
                end_index = Some(i);
                break;
            }
        }
        if state == State::Start {
            state = State::Integer;
        }
    }

    let end = end_index.unwrap_or(str.len());
    let number = &str[..end];
    let value = if dot_index.is_some() {
        number.parse::<f32>().unwrap()
    } else {
        number.parse::<u64>().unwrap() as f32
    };

    Ok((value, &str[end..]))
}

/// Returns whether or not a char is a blankspace (Unicode Pattern_White_Space)
fn is_blankspace(c: char) -> bool {
    c.is_whitespace()
}

/// Returns whether or not a char is a word part (Unicode XID_Continue)
fn is_identifier(c: char) -> bool {
    c == '_' || c.is_alphabetic()
}

fn operator_precedence(token: Token) -> Option<u8> {
    Some(match token {
        Token::Sub => 4,
        Token::Add => 4,
        Token::Div => 3,
        Token::Mul => 2,
        Token::Exp => 1,
        Token::Greater | Token::Lower | Token::GreaterEq | Token::LowerEq | Token::Eq => 5,
        _ => return None,
    })
}

struct Parser<'a> {
    tokens: VecDeque<Token<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(mut str: &'a str) -> Result<Self> {
        let mut tokens = VecDeque::new();
        loop {
            let (tok, rest) = next_token(str)?;
            str = rest;
            if let Token::End = tok {
                break;
            }
            tokens.push_back(tok);
        }
        Ok(Self { tokens })
    }
    fn peek(&self) -> Token {
        self.tokens.front().cloned().unwrap_or(Token::End)
    }
    fn next(&mut self) -> Token {
        self.tokens.pop_front().unwrap_or(Token::End)
    }
    fn ensure_token(&mut self, tok: &Token) -> Result<()> {
        let d = std::mem::discriminant::<Token>(tok);
        let next = self.next();
        let dn = std::mem::discriminant::<Token>(&next);

        if d != dn {
            parse_error!("Expected token {:?}, found {:?}", tok, next)
        } else {
            Ok(())
        }
    }
}

fn parse_monoop(parser: &mut Parser) -> Result<Box<Expression>> {
    let boxed_expr;
    let mut implicit_mul_eligible = false;

    'monop: {
        let expr = match parser.next() {
            Token::Sub => {
                let child = parse_monoop(parser)?;
                Expression::Unary {
                    op: UnaryOperation::Neg,
                    child,
                }
            }
            Token::Number(num) => {
                implicit_mul_eligible = true;
                Expression::Number(num)
            }
            Token::LParen => {
                implicit_mul_eligible = true;
                boxed_expr = parse_expr(parser, 8)?;
                parser.ensure_token(&Token::RParen)?;
                break 'monop;
            }
            Token::Abs => {
                let child = parse_expr(parser, 8)?;
                parser.ensure_token(&Token::Abs)?;
                Expression::Unary {
                    op: UnaryOperation::Abs,
                    child,
                }
            }
            Token::Ident(ident) => {
                if let Some(op) = TernaryOperation::try_function_str(ident) {
                    parser.ensure_token(&Token::LParen)?;
                    let a = parse_expr(parser, 8)?;
                    parser.ensure_token(&Token::Comma)?;
                    let b = parse_expr(parser, 8)?;
                    parser.ensure_token(&Token::Comma)?;
                    let c = parse_expr(parser, 8)?;
                    parser.ensure_token(&Token::RParen)?;

                    Expression::Ternary { op, a, b, c }
                } else if let Some(op) = BinaryOperation::try_function_str(ident) {
                    parser.ensure_token(&Token::LParen)?;
                    let left = parse_expr(parser, 8)?;
                    parser.ensure_token(&Token::Comma)?;
                    let right = parse_expr(parser, 8)?;
                    parser.ensure_token(&Token::RParen)?;

                    Expression::Binary { op, left, right }
                } else if let Some(op) = UnaryOperation::try_function_str(ident) {
                    let child;
                    // handle differently `sin( expr )` and `sin expr`
                    if let Token::LParen = parser.peek() {
                        parser.next();
                        child = parse_expr(parser, 8)?;
                        parser.ensure_token(&Token::RParen)?;
                    } else {
                        child = parse_expr(parser, 2)?;
                    }
                    Expression::Unary { op, child }
                } else if let Some(builtin) = BuiltingVariable::try_from_str(ident) {
                    implicit_mul_eligible = true;
                    Expression::Builtin(builtin)
                } else if let Some(constant) = Constant::try_from_str(ident) {
                    implicit_mul_eligible = true;
                    Expression::Constant(constant)
                } else {
                    return parse_error!("Unknown identifier '{ident}' when parsing monoop");
                }
            }
            other => {
                return parse_error!("Unexpected token '{:?}' in monoop", other);
            }
        };

        boxed_expr = Box::new(expr);
    }

    match parser.peek() {
        Token::Num | Token::LParen | Token::Ident(_) if implicit_mul_eligible => {
            let right = parse_expr(parser, 1)?;
            Ok(Box::new(Expression::Binary {
                op: BinaryOperation::Mul,
                left: boxed_expr,
                right,
            }))
        }
        _ => Ok(boxed_expr),
    }
}

fn parse_expr(parser: &mut Parser, precedence: u8) -> Result<Box<Expression>> {
    let mut left = parse_monoop(parser)?;

    loop {
        let op = parser.peek();

        if let Token::End = op {
            break;
        }

        let bop = BinaryOperation::try_from_token(&op);

        if let Some((op, op_precedence)) = bop {
            if op_precedence <= precedence {
                parser.next();

                left = Box::new(Expression::Binary {
                    op,
                    left,
                    right: parse_expr(parser, op_precedence - 1)?,
                });

                continue;
            }
        }

        break;
    }

    return Ok(left);
}

pub fn debug_ast(node: &Expression) {
    match node {
        Expression::Number(number) => {
            eprint!("{number}");
        }
        Expression::Unary { op, child } => {
            eprint!("({} ", op.name());
            debug_ast(child);
            eprint!(")");
        }
        Expression::Ternary { op, a, b, c } => {
            eprint!("({} ", op.name());
            debug_ast(a);
            eprint!(" ");
            debug_ast(b);
            eprint!(" ");
            debug_ast(c);
            eprint!(")");
        }
        Expression::Binary { op, left, right } => {
            eprint!("({} ", op.name());
            debug_ast(left);
            eprint!(" ");
            debug_ast(right);
            eprint!(")");
        }
        Expression::Builtin(builtin) => {
            eprint!("{:?}", builtin);
        }
        Expression::Constant(constant) => {
            eprint!("{:?}", constant);
        }
        Expression::Variable(var) => {
            eprint!("${var}");
        }
    }
}

pub fn parse_math(expr: &str) -> Result<Box<Expression>> {
    let mut parser = Parser::new(expr)?;
    parse_expr(&mut parser, u8::MAX)
}
