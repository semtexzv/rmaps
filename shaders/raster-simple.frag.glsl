uniform sampler2D u_texture;
in vec2 v_texture_pos;

void main() {
    vec4 color = TEXTURE(u_texture, v_texture_pos);
    color.a = 1.0;

    frag_out = color;
}