uniform mat4 u_matrix;
uniform float u_tex_scale;

in vec2 pos;
out vec2 v_pos;

#pragma property : define float opacity

void main() {
    PASS_FEATURE_IDX;

    #pragma property : init float opacity
    v_pos = pos / u_tex_scale;
    gl_Position = u_matrix *  vec4(pos, 0.0, 1.0);
}