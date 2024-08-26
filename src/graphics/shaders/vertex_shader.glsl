#version 330 core

in vec3 position;
out vec3 vertex_color;
out vec3 vertex_position;

uniform vec3 light;
uniform mat4 view;
uniform mat4 perspective;

void main() {
    vertex_color = position;
    gl_Position = perspective * (view * vec4(position, 1.0));
    vertex_position = position;
}