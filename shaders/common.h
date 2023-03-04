float density(vec4 d);

// compute the gradient of the function using finite differences
vec3 calculate_gradient(vec3 pos) {
    const float d = 0.01;
    
    float x = pos.x;
    float y = pos.y;
    float z = pos.z;
    float t = function_data.time;

#if 0
    float origin = density(vec4(pos, function_data.time));

    float dx = (density(vec4(x + d, y, z, t)) - origin) / d;
    float dy = (density(vec4(x, y + d, z, t)) - origin) / d;
    float dz = (density(vec4(x, y, z + d, t)) - origin) / d;
#else
    float dx = (density(vec4(x + d, y, z, t)) - density(vec4(x - d, y, z, t))) / (2.0 * d);
    float dy = (density(vec4(x, y + d, z, t)) - density(vec4(x, y - d, z, t))) / (2.0 * d);
    float dz = (density(vec4(x, y, z + d, t)) - density(vec4(x, y, z - d, t))) / (2.0 * d);
#endif

    // vec2 n = vec2(1.0, 1.0);
    // if (density(vec4(pos, function_data.time)) < 0.0) {
    //     n = -n; 
    // }
   
    // return vec3(n, -1.0);
    
    return vec3(dx, dy, dz);
}