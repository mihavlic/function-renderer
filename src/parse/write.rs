use crate::parse::{
    self, debug_ast, parse_expr, BinaryOperation, BuiltingVariable, Expression, Parser,
    SsaExpression, Tape,
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

use super::{parser, ParserError};

#[rustfmt::skip]
pub fn math_into_glsl(expr: &str) -> parser::Result<(String, String)> {
    let density = {
        let mut parser = Parser::new(expr)?;
        parse_expr(&mut parser, u8::MAX)?
    };

    let mut tape = Tape::new();
    
    let x = tape.add(SsaExpression::Builtin(BuiltingVariable::X));
    let y = tape.add(SsaExpression::Builtin(BuiltingVariable::Y));
    let z = tape.add(SsaExpression::Builtin(BuiltingVariable::Z));
    let t = tape.add(SsaExpression::Builtin(BuiltingVariable::T));
    
    // select between the function having a shell at the edges of the evaluated range or not
    let last = if true {
        let box_sdf = {
            let str = "
            max(
                max(
                    max(0.5 - x, x - 61.5),
                    max(0.5 - y, y - 61.5)
                ),
                max(0.5 - z, z - 61.5)
            )
            ";
            let mut parser = Parser::new(str)?;
            parse_expr(&mut parser, u8::MAX)?
        };

        let invert = super::Expression::Unary { op: parse::UnaryOperation::Neg, child: density };
        let density = tape.process_ast(&invert);
        let box_sdf = tape.process_ast(&box_sdf);
        tape.add(SsaExpression::Binary { op: BinaryOperation::Max, left: density, right: box_sdf })
    } else {
        tape.process_ast(&density)
    };

    let density = {
        let statements = tape.write_glsl(false).replace("\n", "\n    ");
        format!(
            "float density(vec4 d) {{    
                float {x} = d.x;
                float {y} = d.y;
                float {z} = d.z;
                float {t} = d.w;

                {statements}
                return {last};
            }}"
        ).replace("\n            ", "\n")
    };

    let diff = {
        let statements = tape.write_glsl(true).replace("\n", "\n    ");
        format!(
            "vec4 gradient_density(vec4 d) {{    
                float {x} = d.x;
                float {y} = d.y;
                float {z} = d.z;
                float {t} = d.w;

                {statements}
                return vec4({last}d, {last});
            }}"
        ).replace("\n            ", "\n")
    };

    Ok((density, diff))
}

pub struct GlslCompiler {
    compiler: shaderc::Compiler,
    options: shaderc::CompileOptions<'static>,
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
