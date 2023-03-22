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

pub const MIN_MARGIN: f32 = 0.5;
pub const MAX_MARGIN: f32 = 2.5;

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

    let xn = tape.add(SsaExpression::Builtin(BuiltingVariable::X_normalized));
    let yn = tape.add(SsaExpression::Builtin(BuiltingVariable::Y_normalized));
    let zn = tape.add(SsaExpression::Builtin(BuiltingVariable::Z_normalized));
    
    // select between the function having a shell at the edges of the evaluated range or not
    let min_margin =  MIN_MARGIN / 63.0;
    let max_margin = 1.0 - MAX_MARGIN / 63.0;

    let last = tape.process_ast(&density);

    let density = {
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
                
                float minm = {min_margin};
                float maxm = {max_margin};
                float box_sdf = max(
                    max(
                        max(minm - {xn}, {xn} - maxm),
                        max(minm - {yn}, {yn} - maxm)
                    ),
                    max(minm - {zn}, {zn} - maxm)
                );

                return max(box_sdf, -{last});
            }}"
        ).replace("\n            ", "\n")
    };

    let diff = {
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

                {last} = -{last};
                {last}d = -{last}d;

                float minm = {min_margin};
                float maxm = {max_margin};

                float x1 = minm - {xn};
                float x2 = {xn} - maxm;
                float xmax;
                vec3 xmaxd;
                if (x1 < x2) {{
                    xmax = x2;
                    xmaxd = vec3(1.0, 0.0, 0.0);
                }} else {{
                    xmax = x1;
                    xmaxd = vec3(-1.0, 0.0, 0.0);
                }}

                float y1 = minm - {yn};
                float y2 = {yn} - maxm;
                float ymax;
                vec3 ymaxd;
                if (y1 < y2) {{
                    ymax = y2;
                    ymaxd = vec3(0.0, 1.0, 0.0);
                }} else {{
                    ymax = y1;
                    ymaxd = vec3(0.0, -1.0, 0.0);
                }}

                float xymax;
                vec3 xymaxd;
                if (xmax < ymax) {{
                    xymax = ymax;
                    xymaxd = ymaxd;
                }} else {{
                    xymax = xmax;
                    xymaxd = xmaxd;
                }}

                float z1 = minm - {zn};
                float z2 = {zn} - maxm;
                float zmax;
                vec3 zmaxd;
                if (y1 < z2) {{
                    zmax = z2;
                    zmaxd = vec3(0.0, 0.0, 1.0);
                }} else {{
                    zmax = z1;
                    zmaxd = vec3(0.0, 0.0, -1.0);
                }}

                float xyzmax;
                vec3 xyzmaxd;
                if (xymax < zmax) {{
                    xyzmax = zmax;
                    xyzmaxd = zmaxd;
                }} else {{
                    xyzmax = xymax;
                    xyzmaxd = xymaxd;
                }}

                float fmax;
                vec3 fmaxd;
                // the last max is somewhat odd because we want to apply the box sdf only where it positive - ie. outside the box (there is some negative bias)
                // otherwise the interpolated position may get \"under\" the implicit surface where the box sdf density may be higher
                // and so it will also get its derivative which leads to visual artifacts  
                if (xyzmax >= -0.001) {{
                    fmax = xyzmax;
                    fmaxd = xyzmaxd;
                }} else {{
                    fmax = {last};
                    fmaxd = {last}d;
                }}

                return vec4(fmaxd, fmax);
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
