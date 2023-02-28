#version 450
#extension GL_ARB_separate_shader_objects : enable
#extension GL_EXT_scalar_block_layout : enable

layout(push_constant, scalar) uniform PushConstant { 
    mat4 model_matrix;
    mat4 projection_matrix;
} push;

layout(location = 0) in vec3 inPosition;
layout(location = 1) in vec4 inNormal;

layout(location = 0) out vec3 outWorldPos;
layout(location = 1) out vec3 outWorldNormal;
layout(location = 2) out vec3 outViewNormal;
layout(location = 3) out vec3 outViewPos;

void main() {
    vec4 pos = push.projection_matrix * push.model_matrix * vec4(inPosition, 1.0);

    outWorldPos = inPosition;
    // FIXME for some reason normals arrive flipped?
    outWorldNormal = inNormal.yxz;
    outViewNormal = (push.model_matrix * vec4(inPosition + inNormal.xyz, 1.0) ).xyz - (push.model_matrix * vec4(inPosition, 1.0)).xyz;
    outViewPos = pos.xyz;
    gl_Position = pos;
}