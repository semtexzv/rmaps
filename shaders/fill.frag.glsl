#pragma property : define vec4 color;
#pragma property : define float opacity;


void main() {
    PASS_FEATURE_IDX;

    #pragma property : init vec4 color;
    #pragma property : init float opacity;

    frag_out = color * opacity;
}