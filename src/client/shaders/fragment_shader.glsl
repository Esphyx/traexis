#version 330 core

in vec3 vertex_position;
in vec3 vertex_color;
in vec2 vertex_uv;
out vec4 color;

uniform float width;
uniform float height;
uniform float depth;

uniform sampler2D atlas;

void main() {
    if (vertex_uv != vec2(0, 0)) {
        color = texture(atlas, vertex_uv);
    } else {
        color = vec4(vertex_color, 1.0);
    }

}