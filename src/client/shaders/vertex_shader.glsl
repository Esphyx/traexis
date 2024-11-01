#version 330 core

in vec3 position;
in vec3 color;
in vec2 uv;

out vec3 vertex_position;
out vec3 vertex_color;
out vec2 vertex_uv; 

uniform mat4 view;
uniform mat4 perspective;
void main() {
    gl_Position = perspective * view * vec4(position, 1.0);
    vertex_position = position;
    vertex_color = color;
    vertex_uv = uv;
}