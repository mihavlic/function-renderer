#extension GL_EXT_scalar_block_layout : enable

struct Vertex {
    vec3 position;
};

layout(binding = 0, r16f) uniform image3D function_values;
layout(binding = 1, rgba32f) uniform image3D intersections;
layout(binding = 2, r32ui) uniform uimage3D vertex_indices;
layout(binding = 3, scalar) buffer VertexData {
    uint size;
    uint offset;
	Vertex vertices[];
} vertices;
layout(binding = 4, scalar) buffer IndexData {
    uint size;
    uint offset;
	uint indices[];
} indices;
layout(binding = 5, scalar) uniform FunctionData {
    vec3 rect_min;
    vec3 rect_max;
    float time;
} function_data;

vec4 normalized_pos_to_density_input(vec3 normalized_pos) {
    vec3 world_pos = mix(function_data.rect_min, function_data.rect_max, normalized_pos);
    return vec4(world_pos, function_data.time);
}
