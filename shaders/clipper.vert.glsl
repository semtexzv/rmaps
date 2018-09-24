uniform mat4 u_matrix;

in vec2 pos;


void main() {
    PASS_FEATURE_IDX;

    gl_Position = u_matrix *  vec4(pos, 0.0, 1.0);
}