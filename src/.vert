#version 330 core

layout (location = 0) in vec3 Position;

uniform vec2 u_resolution;
uniform mat4 u_model_matrix;
uniform mat4 u_view_matrix;
uniform mat4 u_proj_matrix;

out vec4 fragmentColor;

void main()
{
    vec4 uv = u_proj_matrix * u_view_matrix * u_model_matrix * vec4(Position, 1.0);
    float zDepth = 1 - 0.1 * (uv.z - 4);
    float bDepth = 1 - 0.4 * (uv.z - 4);

    if (u_resolution.x > u_resolution.y) {
        uv.x *= u_resolution.y / u_resolution.x;
    } else {
        uv.y *= u_resolution.x / u_resolution.y;
    }

    fragmentColor = vec4(bDepth * Position.x, 0.5 * (zDepth + bDepth) * Position.y, max(0, zDepth) * Position.z * 0.92 + 0.08, zDepth);
    gl_Position = uv;
}