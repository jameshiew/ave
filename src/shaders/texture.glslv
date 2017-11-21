#version 150

in vec3 position;
in vec3 normal;
in vec2 tex_coords;

out vec3 v_normal;
out vec3 v_position;
out vec2 v_tex_coords;

uniform mat4 perspective;
uniform mat4 view;
uniform mat4 model;

void main() {
    v_tex_coords = tex_coords;
    mat4 modelview = view * model;
    v_normal = transpose(inverse(mat3(modelview))) * normal;
    gl_Position = perspective * modelview * vec4(position, 1.0);
    v_position = gl_Position.xyz / gl_Position.w;
}
