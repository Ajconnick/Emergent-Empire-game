#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 vertexColor;

uniform vec2 u_resolution;
uniform mat4 u_model_matrix;
uniform mat4 u_view_matrix;
uniform mat4 u_proj_matrix;

out vec4 fragmentColor;

void main()
{
    vec4 uv = u_proj_matrix * u_view_matrix * u_model_matrix * vec4(Position, 1.0);

    if (u_resolution.x > u_resolution.y) {
        uv.x *= u_resolution.y / u_resolution.x;
    } else {
        uv.y *= u_resolution.x / u_resolution.y;
    }

    fragmentColor = vec4(vertexColor, 1.0);
    gl_Position = uv;
}