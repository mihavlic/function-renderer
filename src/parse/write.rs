use crate::{
    embed::MaybeFile,
    parse::{self, BinaryOperation, BuiltingVariable, SsaExpression, Tape},
};
use pumice::vk;
use shaderc::ShaderKind;
use std::{error::Error, path::Path};

use super::{parse_math, parser, TotalF32, UnaryOperation};

pub const MIN_MARGIN: f32 = 0.8;
pub const MAX_MARGIN: f32 = 2.5;

#[rustfmt::skip]
pub fn math_into_glsl(expr: &str, thickness: bool) -> parser::Result<(String, String)> {
    let mut tape = Tape::new();
    
    let x = tape.add(SsaExpression::Builtin(BuiltingVariable::X));
    let y = tape.add(SsaExpression::Builtin(BuiltingVariable::Y));
    let z = tape.add(SsaExpression::Builtin(BuiltingVariable::Z));
    let t = tape.add(SsaExpression::Builtin(BuiltingVariable::T));

    let xn = tape.add(SsaExpression::Builtin(BuiltingVariable::normalized_X));
    let yn = tape.add(SsaExpression::Builtin(BuiltingVariable::normalized_Y));
    let zn = tape.add(SsaExpression::Builtin(BuiltingVariable::normalized_Z));
    
    let min_margin = MIN_MARGIN / 63.0;
    let max_margin = 1.0 - MAX_MARGIN / 63.0;

    let density = {
        let expr = parse_math(expr)?;
        tape.add_ast(&expr)
    };
    
    let last = if thickness {
        let box_sdf = {
            let expr = format!("
                max(
                    max(
                        max({min_margin} - normalized_x, normalized_x - {max_margin}),
                        max({min_margin} - normalized_y, normalized_y - {max_margin})
                    ),
                    max({min_margin} - normalized_z, normalized_z - {max_margin})
                )
            ");
            let expr = parse_math(&expr).unwrap();
            tape.add_ast(&expr)
        };

        let density = tape.add(SsaExpression::Unary { op: UnaryOperation::Neg, child: density });

        // we want to apply the box sdf only where it's positive - ie. outside the box (there is some
        // conservative bias) otherwise the interpolated position may get \"under\" the implicit surface
        // where the box sdf density may be higher and so it will also get its derivative which leads to
        // visual artifacts  
        let epsilon = tape.add(SsaExpression::Constant(TotalF32(0.001)));
        let biased = tape.add(SsaExpression::Binary { op: BinaryOperation::Add, left: box_sdf, right: epsilon });
        let last = tape.add(SsaExpression::Ternary { op: parse::TernaryOperation::Select, a: biased, b: box_sdf, c: density });

        last
    } else {
        density
    };

    let function = {
        let statements = tape.write_glsl(false).replace("\n", "\n    ");
        format!(
            "float density(vec4 d, vec3 n) {{    
                float {x} = d.x;
                float {y} = d.y;
                float {z} = d.z;
                float {t} = d.w;

                float {xn} = n.x;
                float {yn} = n.y;
                float {zn} = n.z;

                {statements}                
                return {last};
            }}"
        ).replace("\n            ", "\n")
    };

    let gradient = {
        let statements = tape.write_glsl(true).replace("\n", "\n    ");
        format!(
            "vec4 gradient_density(vec4 d, vec3 n) {{    
                float {x} = d.x;
                float {y} = d.y;
                float {z} = d.z;
                float {t} = d.w;

                float {xn} = n.x;
                float {yn} = n.y;
                float {zn} = n.z;

                {statements}
                return vec4({last}d, {last});
            }}"
        ).replace("\n            ", "\n")
    };

    Ok((function, gradient))
}

/// A wrapper around [`shaderc::Compiler`] which handles our specific configuration and preprocessor include callback.
pub struct GlslCompiler {
    compiler: shaderc::Compiler,
    options: shaderc::CompileOptions<'static>,
}

impl GlslCompiler {
    pub fn new(includes: &[(&str, MaybeFile)]) -> Self {
        let includes = includes
            .into_iter()
            .map(|(name, file)| ((*name).to_owned(), *file))
            .collect::<Vec<_>>();

        let compiler = shaderc::Compiler::new().unwrap();
        let mut options = shaderc::CompileOptions::new().unwrap();
        options.set_target_env(shaderc::TargetEnv::Vulkan, vk::API_VERSION_1_1);
        options.set_generate_debug_info();
        options.set_include_callback(move |name, _, _, _| {
            let found = includes
                .iter()
                .find(|(include_name, _)| *include_name == name);

            let Some((_, file)) = found else {
                return Err(format!("'{name:?}' does not exist"));
            };

            let Ok(content) = file.read() else {
                return Err(format!("'{:?}' is not a file", file.path()));
            };

            Ok(shaderc::ResolvedInclude {
                resolved_name: file.as_str().to_owned(),
                content: std::str::from_utf8(&content).unwrap().to_string(),
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
    /// Compile glsl source code to spirv.
    ///
    /// * `source` - The glsl source code.
    /// * `file_name` - A file name for shader kind detection and error identification, doesn't have to be a valid path.
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
    /// Set a preprocessor define in the compiler.
    pub fn set_define(&mut self, name: &str, value: Option<&str>) {
        self.options.add_macro_definition(name, value);
    }
}
