uniform mat4 u_matrix;

in vec2 pos;

#pragma property : define vec4 color
#pragma property : define float opacity

void main() {
    PASS_FEATURE_IDX;

    #pragma property : init vec4 color
    #pragma property : init float opacity

    gl_Position = u_matrix *  vec4(pos, 0.0, 1.0);
}