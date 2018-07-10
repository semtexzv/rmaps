#version 100

uniform sampler2D u_texture;
in vec2 v_texture_pos;

void main() {
    gl_FragColor = vec4(texture2D(u_texture,v_texture_pos).rgb,0.5);
}