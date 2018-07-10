#version 100

uniform highp mat4 u_matrix;

in vec2 pos;
in vec2 tex;
out vec2 v_texture_pos;

void main() {
    gl_Position = u_matrix * vec4(pos, 0.0, 1.0);
    v_texture_pos = tex;
}