
layout(std140) uniform feature_data_ubo {
    vec4 feature_data[1024];
};

float feature_get_float(float idx, float offset) {
    return feature_data[int(idx * PER_FEATURE_SIZE + offset)].x;
}

vec2 feature_get_vec2(float idx, float offset) {
    return feature_data[int(idx * PER_FEATURE_SIZE + offset)].xy;
}

vec3 feature_get_vec3(float idx, float offset) {
    return feature_data[int(idx * PER_FEATURE_SIZE + offset)].xyz;
}

vec4 feature_get_vec4(float idx, float offset) {
    return feature_data[int(idx * PER_FEATURE_SIZE + offset)].xyzw;
}

#define TEXTURE texture