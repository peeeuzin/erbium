#version 450
layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec2 texture;

uniform mat4 persp_matrix;
uniform mat4 view_matrix;

out vec3 frag_normal;
out vec2 frag_texture;
out vec3 frag_position;

void main() {
    gl_Position = persp_matrix * view_matrix * vec4(position, 1.0);

    frag_normal = normal;
    frag_texture = texture;
    frag_position = position;
}