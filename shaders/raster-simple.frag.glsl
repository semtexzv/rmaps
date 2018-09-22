uniform sampler2D u_texture;
in vec2 v_texture_pos;

void main() {
    frag_out = vec4(texture2D(u_texture,v_texture_pos).rgb,1.0);
}