#version 450
#extension GL_ARB_separate_shader_objects : enable
#extension GL_EXT_scalar_block_layout : enable

layout(push_constant, scalar) uniform PushConstant { 
    mat4 matrix;
} push;

layout(location = 0) in vec3 inPosition;

void main() {
    gl_Position = push.matrix * vec4(inPosition, 1.0);
}