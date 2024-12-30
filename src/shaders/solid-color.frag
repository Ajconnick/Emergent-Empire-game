#version 330 core

uniform vec4 u_color;

out vec4 Color;

void main()
{
    Color = u_color;
}