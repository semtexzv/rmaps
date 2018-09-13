uniform mat4 u_matrix;
uniform float u_tex_scale;

uniform vec2 u_pattern_tl;
uniform vec2 u_pattern_br;
uniform vec2 u_texsize;


in vec2 pos;
out vec2 v_pos;

#pragma property : define float opacity

/*
vec2 get_pattern_pos(const vec2 pixel_coord_upper, const vec2 pixel_coord_lower,
    const vec2 pattern_size, const float tile_units_to_pixels, const vec2 pos) {

    vec2 offset = mod(mod(mod(pixel_coord_upper, pattern_size) * 256.0, pattern_size) * 256.0 + pixel_coord_lower, pattern_size);
    return (tile_units_to_pixels * pos + offset) / pattern_size;
}
*/

void main() {
    PASS_FEATURE_IDX;

    #pragma property : init float opacity
    v_pos = pos / u_tex_scale;
    gl_Position = u_matrix *  vec4(pos, 0.0, 1.0);
}