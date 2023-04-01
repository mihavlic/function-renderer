#version 450
#extension GL_EXT_scalar_block_layout : enable

layout(binding = 0, scalar) uniform FunctionData {
    mat4 model_matrix;
    mat4 projection_matrix;
    vec3 rect_min;
    vec3 rect_max;
    float time;
    vec3 camera_dir;
} data;

layout(location = 0) in vec3 inPosition;

layout(location = 0) out vec3 outWorldPos;
layout(location = 1) out vec3 outViewPos;

void main() {
    vec4 pos = data.projection_matrix * data.model_matrix * vec4(inPosition, 1.0);

    outWorldPos = inPosition;
    outViewPos = pos.xyz;
    gl_Position = pos;
}