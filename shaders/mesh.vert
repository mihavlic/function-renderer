#version 450
#extension GL_EXT_scalar_block_layout : enable

layout(push_constant, scalar) uniform PushConstant { 
    mat4 model_matrix;
    mat4 projection_matrix;
} push;

layout(location = 0) in vec3 inPosition;

layout(location = 0) out vec3 outWorldPos;
layout(location = 1) out vec3 outViewPos;

void main() {
    vec4 pos = push.projection_matrix * push.model_matrix * vec4(inPosition, 1.0);

    outWorldPos = inPosition;
    outViewPos = pos.xyz;
    gl_Position = pos;
}