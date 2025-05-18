#version 450

layout(location = 0) out vec3 frag_normal;
layout(location = 1) out vec3 frag_texture;
layout(location = 2) out vec3 frag_position;

out vec4 FragColor;

const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);

void main() {
    float lum = max(dot(normalize(frag_normal), normalize(LIGHT)), 0.0);
    vec3 color = (0.3 + 0.7 * lum) * vec3(1.0, 1.0, 1.0);
    FragColor = vec4(color, 1.0);
}
