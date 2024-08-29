#version 330 core

in vec3 vertex_color;
in vec3 vertex_position;

uniform vec3 light;

void main() {
    float luminence = dot(vertex_position, light);
    gl_FragColor = luminence * vec4(vertex_color, 1.0);
}