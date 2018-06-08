#version 100
varying highp vec3 vColor;
void main() {
    gl_FragColor = vec4(vColor, 1.0);
}