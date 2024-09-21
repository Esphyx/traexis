#version 330 core

in vec3 vertex_position;
in vec3 vertex_color;

uniform float width;
uniform float height;
uniform float depth;

void main() {
    vec3 scaled = vec3(vertex_position.x / width, vertex_position.y / height, vertex_position.z / depth);
    gl_FragColor = vec4(scaled, 1.0);
}