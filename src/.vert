#version 330 core

layout (location = 0) in vec3 Position;

// uniform vec2 u_resolution;
// uniform mat4 u_model_matrix;
// uniform mat4 u_view_matrix;
// uniform mat4 u_proj_matrix;

void main()
{
    // vec4 uv = u_proj_matrix * u_view_matrix * u_model_matrix * vec4(Position, 1.0);

    // if (u_resolution.x > u_resolution.y) {
    //     uv.x *= u_resolution.y / u_resolution.x;
    // } else {
    //     uv.y *= u_resolution.x / u_resolution.y;
    // }

    gl_Position = vec4(Position, 1.0);
}