#pragma property : define vec4 color;
#pragma property : define float opacity;


void main() {

    #pragma property : init vec4 color;
    #pragma property : init float opacity;

    gl_FragColor = color * opacity;
}