use crate::parser::{
    self, array_eval, debug_ast, parse_expr, BinaryOperation, Expression, Lane, Parser,
};
use graph::device::read_spirv;
use std::{
    error::Error,
    f32::consts::FRAC_PI_2,
    fmt::Write,
    fs::File,
    path::Path,
    process::{ExitStatus, Stdio},
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
"float density(vec4 data) {{
    const float CONST_E = 2.71828182845904523536028747135266250;
    const float CONST_PI = 3.14159265358979323846264338327950288;
    const float CONST_HALF_PI = 1.57079632679489661923132169163975144;
    
    float x = data.x;
    float y = data.y;
    float z = data.z;
    float t = data.w;

    return {expr};
}}")
}

// pub fn make_shader_with_density(path: )

pub fn compile_glsl_to_spirv(
    source_code: &str,
    temp_folder: &Path,
    ending: &str,
) -> Result<Vec<u32>, Box<dyn Error>> {
    let file = format!("source.{ending}");

    let source = temp_folder.join(&file);
    std::fs::write(&source, source_code)?;

    let child = std::process::Command::new("glslangValidator")
        .arg("--target-env")
        .arg("vulkan1.1")
        // -gVS seems to break reflection?
        .arg("-g")
        .arg("-o")
        .arg("compiled.spv")
        .arg(&file)
        .current_dir(temp_folder)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to run glslangValidator");

    let output = child.wait_with_output()?;

    if !output.status.success()
        || std::str::from_utf8(&output.stdout)
            .map(|str| str.contains("error"))
            .unwrap_or(false)
    {
        let mut err = String::from_utf8_lossy(&output.stderr).to_string();
        err += &String::from_utf8_lossy(&output.stdout);
        Err(err)?;
    }

    let mut compiled = File::open(temp_folder.join("compiled.spv"))?;

    read_spirv(&mut compiled).map_err(|e| e.into())
}

// pub fn compile_wgsl_to_spirv(source_code: &str, temp_folder: &Path) -> Vec<u32> {
//     let source = temp_folder.join("source.wgsl");
//     std::fs::write(&source, source_code).unwrap();

//     let child = std::process::Command::new("naga")
//         .arg("source.wgsl")
//         .arg("compiled.spv")
//         .current_dir(temp_folder)
//         .spawn()
//         .expect("Failed to run glslangValidator");
//     let output = child.wait_with_output();

//     let mut compiled = File::open(temp_folder.join("compiled.spv")).unwrap();

//     read_spirv(&mut compiled).unwrap()
// }
