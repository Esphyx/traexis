#version 330 core

in vec3 position;
in vec3 color;
out vec3 vertex_position;
out vec3 vertex_color;

uniform mat4 view;
uniform mat4 perspective;
void main() {
    gl_Position = perspective * view * vec4(position, 1.0);
    vertex_position = position;
    vertex_color = color;
}