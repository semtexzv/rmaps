uniform mat4 u_matrix;

in vec2 pos;
in vec4 col;
in float opacity;

out vec4 v_color;
out float v_opacity;

void main() {
    gl_Position = u_matrix *  vec4(pos, 0.0, 1.0);
    v_color =  col;

    v_opacity = 1.0;
}