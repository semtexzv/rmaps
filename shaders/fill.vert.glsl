#version 100

uniform highp mat4 matrix;
attribute highp vec2 position;
attribute highp vec3 color;

varying highp vec3 vColor;

void main() {
    gl_Position = vec4(position, 0.0, 1.0) * matrix;
    vColor = color;
}