uniform vec2 u_texsize;
uniform sampler2D u_image;

in vec2 v_pos;

uniform vec2 u_pattern_tl;
uniform vec2 u_pattern_br;

#pragma property : define float opacity;

void main() {
    PASS_FEATURE_IDX;

    #pragma property : init float opacity;
    // Get repeating tex coord
    vec2 coord = mod(v_pos,1.0);

    vec2 start = u_pattern_tl / u_texsize;
    vec2 end = u_pattern_br / u_texsize;

    vec2 tex_coord = mix(start, end, coord);
    vec4 texel_color = TEX_LOOKUP(u_image, tex_coord);

    frag_out = texel_color * vec4(1.0,1.0,1.0,opacity);
}