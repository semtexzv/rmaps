#pragma property : define vec4 color;
#pragma property : define float opacity;

in vec2 v_normal;

void main() {
    PASS_FEATURE_IDX;

    #pragma property : init vec4 color;
    #pragma property : init float opacity;

    frag_out = color;//vec4(v_normal.x,v_normal.y,0.0,1.0);
}