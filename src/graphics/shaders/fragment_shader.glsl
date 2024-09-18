#version 330 core

in vec3 vertex_color;
in vec3 vertex_position;

void main() {
    gl_FragColor = vec4(vertex_color, 1.0);
}