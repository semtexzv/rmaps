uniform mat4 u_matrix;

in vec2 pos;
in vec4 col;
in float opacity;

out vec4 v_color;
out float v_opacity;


float rand(vec2 co){
    return fract(sin(dot(co.xy ,vec2(12.9898,78.233))) * 43758.5453);
}

void main() {
    gl_Position = u_matrix *  vec4(pos, 0.0, 1.0) ;
    v_color =  col * vec4(rand(pos),rand(pos),rand(pos),0.3);

    v_opacity = 1.0;
}