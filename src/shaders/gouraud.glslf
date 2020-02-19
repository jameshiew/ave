#version 450

in vec3 v_normal;
in vec3 v_color;

out vec4 color;

const vec3 light = vec3(1.0, 1.0, 1.0);
const vec3 dark = vec3(0.0, 0.0, 0.0);

void main() {
    float brightness = dot(normalize(v_normal), normalize(light));
    color = vec4(mix(dark, v_color, brightness), 1.0);
}
