in vec4 v_color;
in float v_opacity;


void main() {
    gl_FragColor = v_color * v_opacity;
}