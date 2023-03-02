struct Vertex {
    vec3 position; 
    uint packed_normal;
};

layout(binding = 0, r16f) uniform image3D function_values;
layout(binding = 1, rgba8) uniform image3D intersections;
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
	uvec3 size;
    vec3 offset;
    float scale;
    float time;
} function_data;
