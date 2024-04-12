#version 330 core

in vec4 fragmentColor;

out vec4 Color;

uniform float u_time;

void main()
{
    Color = fragmentColor;
}