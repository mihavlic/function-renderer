use crate::parser::{
    self, array_eval, debug_ast, parse_expr, BinaryOperation, Expression, Lane, Parser,
};
use graph::device::read_spirv;
use pumice::vk;
use shaderc::{IncludeCallbackResult, ShaderKind};
use std::{
    error::Error,
    f32::consts::FRAC_PI_2,
    fmt::Write,
    fs::File,
    path::{Path, PathBuf},
    process::{ExitStatus, Stdio},
    str::FromStr,
};

pub fn write_glsl(node: &Expression) -> String {
    let mut out = String::new();
    match node {
        Expression::Binary { op, left, right } => {
            let left = write_glsl(left);
            let right = write_glsl(right);
            match op {
                BinaryOperation::Sub
                | BinaryOperation::Add
                | BinaryOperation::Div
                | BinaryOperation::Mul => {
                    let c = match op {
                        BinaryOperation::Sub => '-',
                        BinaryOperation::Add => '+',
                        BinaryOperation::Div => '/',
                        BinaryOperation::Mul => '*',
                        _ => unreachable!(),
                    };
                    write!(out, "({left} {c} {right})").unwrap();
                }
                BinaryOperation::Exp => {
                    write!(out, "pow({left}, {right})").unwrap();
                }
                BinaryOperation::Greater
                | BinaryOperation::Lower
                | BinaryOperation::GreaterEq
                | BinaryOperation::LowerEq
                | BinaryOperation::Eq => {
                    let cmp = match op {
                        BinaryOperation::Greater => ">",
                        BinaryOperation::Lower => "<",
                        BinaryOperation::GreaterEq => ">=",
                        BinaryOperation::LowerEq => "<=",
                        BinaryOperation::Eq => "==",
                        _ => unreachable!(),
                    };
                    write!(out, "((float)({left} {cmp} {right}))").unwrap();
                }
            }
        }
        Expression::Unary { op, child } => 'block: {
            let child = write_glsl(child);
            let name = match op {
                parser::UnaryOperation::Neg => {
                    write!(out, "-{child}").unwrap();
                    break 'block;
                }
                parser::UnaryOperation::Log => {
                    write!(out, "log({child}) / log(10.0)").unwrap();
                    break 'block;
                }
                parser::UnaryOperation::Log2 => "log2",
                parser::UnaryOperation::Ln => "log",
                parser::UnaryOperation::Sqrt => "sqrt",
                parser::UnaryOperation::Sin => "sin",
                parser::UnaryOperation::Cos => "cos",
                parser::UnaryOperation::Tan => "tan",
                parser::UnaryOperation::Abs => "abs",
                parser::UnaryOperation::CoTan => {
                    write!(out, "1.0 / tan({child})").unwrap();
                    break 'block;
                }
                parser::UnaryOperation::ArcSin => "asin",
                parser::UnaryOperation::ArcCos => "scos",
                parser::UnaryOperation::ArcTan => "atan",
                parser::UnaryOperation::Length => "length",
                parser::UnaryOperation::ArcCotan => {
                    write!(out, "CONSTANT_HALF_PI - atan({child})").unwrap();
                    break 'block;
                }
            };
            write!(out, "{name}({child})").unwrap();
        }
        Expression::Builtin(b) => {
            let val = match b {
                parser::BuiltingVariable::X => 'x',
                parser::BuiltingVariable::Y => 'y',
                parser::BuiltingVariable::Z => 'z',
                parser::BuiltingVariable::T => 't',
            };
            out.push(val);
        }
        Expression::Constant(c) => {
            std::f32::consts::E;
            let c = match c {
                parser::Constant::E => "CONSTANT_E",
                parser::Constant::Pi => "CONSTANT_PI",
            };
            out.push_str(c);
        }
        Expression::Variable(_) => todo!(),
        Expression::Number(val) => {
            write!(out, "{:?}", val).unwrap();
        }
    }
    out
}

pub fn make_glsl_math(raw_math: &str) -> String {
    let mut parser = Parser::new(raw_math);
    let root = parse_expr(&mut parser, u8::MAX);

    // eprint!("Debug ast of {}:  ", raw_math.trim());
    // debug_ast(&root);
    // eprintln!();

    write_glsl(&root)
}

#[rustfmt::skip]
pub fn make_density_function(expr: &str) -> String {
    format!(
"float density(vec4 d) {{
    const float CONSTANT_E = 2.71828182845904523536028747135266250;
    const float CONSTANT_PI = 3.14159265358979323846264338327950288;
    const float CONSTANT_HALF_PI = 1.57079632679489661923132169163975144;
    
    float x = d.x;
    float y = d.y;
    float z = d.z;
    float t = d.w;

    if (
        x <= 0.5 || x >= 61.5 ||
        y <= 0.5 || y >= 61.5 ||
        z <= 0.5 || z >= 61.5
    ) {{
        return 0.1;
    }} else {{
        return {expr};
    }}
}}")
}

pub struct GlslCompiler {
    compiler: shaderc::Compiler,
    options: shaderc::CompileOptions<'static>,
}

pub fn math_into_glsl(math: &str) -> std::thread::Result<String> {
    let glsl_math = std::panic::catch_unwind(|| make_glsl_math(&math))?;
    Ok(make_density_function(&glsl_math))
}

impl GlslCompiler {
    pub fn new() -> Self {
        let compiler = shaderc::Compiler::new().unwrap();
        let mut options = shaderc::CompileOptions::new().unwrap();
        options.set_target_env(shaderc::TargetEnv::Vulkan, vk::API_VERSION_1_1);
        options.set_generate_debug_info();
        options.set_include_callback(|name, _, _, _| {
            let path = PathBuf::from_str("shaders").unwrap().join(name);
            let Ok(full) = path
                .canonicalize()  else {
                    return Err(format!("'{path:?}' does not exist"));
                };

            let Ok(content) = std::fs::read_to_string(&full) else {
                return Err(format!("'{path:?}' is not a file"));
            };

            Ok(shaderc::ResolvedInclude {
                resolved_name: full.to_str().unwrap().to_owned(),
                content,
            })
        });

        Self { compiler, options }
    }
    pub fn compile_file(&self, path: &impl AsRef<Path>) -> Result<Vec<u32>, Box<dyn Error>> {
        let path = path.as_ref();
        let Ok(source) = std::fs::read_to_string(path) else {
            return Err(format!("'{path:?}' couldn't be opened").into()); 
        };

        let spirv = self.compile(&source, path.file_name().unwrap().to_str().unwrap())?;
        Ok(spirv)
    }
    pub fn compile(&self, source: &str, file_name: &str) -> shaderc::Result<Vec<u32>> {
        let kind = match file_name.rsplit('.').next().unwrap() {
            "comp" => ShaderKind::Compute,
            "vert" => ShaderKind::Vertex,
            "frag" => ShaderKind::Fragment,
            _ => panic!("Unknown shader file kind for '{file_name}'"),
        };

        self.compiler
            .compile_into_spirv(source, kind, file_name, "main", Some(&self.options))
            .map(|artifact| artifact.as_binary().to_vec())
    }
    pub fn set_define(&mut self, name: &str, value: Option<&str>) {
        self.options.add_macro_definition(name, value);
    }
}
