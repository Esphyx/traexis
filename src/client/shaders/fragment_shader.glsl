#version 330 core

in vec3 vertex_position;
in vec3 vertex_color;

uniform float width;
uniform float height;
uniform float depth;

void main() {
    gl_FragColor = vec4(vertex_color, 1.0);
}