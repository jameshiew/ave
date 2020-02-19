#version 450

in vec3 position;
in vec3 color;
in vec3 normal;

out vec3 v_color;
out vec3 v_normal;

uniform mat4 perspective;
uniform mat4 view;
uniform mat4 model;

void main() {
    mat4 modelview = view * model;
    gl_Position = perspective * modelview * vec4(position, 1.0);
    v_normal = normal;
    v_color = color;
}
