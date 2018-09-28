uniform mat4 u_matrix;
uniform vec2 u_texsize;

uniform float u_tex_scale;

uniform vec2 u_pattern_tl;
uniform vec2 u_pattern_br;


in vec2 pos;
out vec2 v_pos;

#pragma property : define float opacity


void main() {
    PASS_FEATURE_IDX;

    #pragma property : init float opacity
    v_pos = pos / u_tex_scale;
    gl_Position = u_matrix *  vec4(pos, 0.0, 1.0);
}