#version 330 core

uniform mat4 u_model_matrix;
uniform mat4 u_view_matrix;
uniform mat4 u_proj_matrix;

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Normal_modelspace;
layout (location = 2) in vec3 texture_coord;
layout (location = 3) in vec3 Color;

out vec3 uv;

void main()
{
    gl_Position = u_proj_matrix * u_view_matrix * u_model_matrix * vec4(Position, 1.0);
    uv = texture_coord;
}