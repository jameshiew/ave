#version 450

in vec3 v_normal;
in vec3 v_position;
in vec3 v_color;

out vec4 color;

const vec3 light = vec3(0.1, 1.0, 1.0);
const vec3 diffuse_color = vec3(0.2, 0.2, 0.2);
const vec3 specular_color = vec3(0.5, 0.5, 0.5);

void main() {
    float diffuse = max(dot(normalize(v_normal), normalize(light)), 0.0);

    vec3 camera_dir = normalize(-v_position);
    vec3 half_direction = normalize(normalize(light) + camera_dir);
    float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);

    color = vec4(v_color + diffuse * diffuse_color + specular * specular_color, 1.0);
}
