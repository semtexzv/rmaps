uniform mat4 u_matrix;

in vec2 pos;

#pragma property : define vec4 color
#pragma property : define float opacity


/*
float rand(vec2 co){
    return fract(sin(dot(co.xy ,vec2(12.9898,78.233))) * 43758.5453);
}
*/

void main() {
    v_feature = feature;
    #pragma property : init vec4 color
    #pragma property : init float opacity

    gl_Position = u_matrix *  vec4(pos, 0.0, 1.0);
}