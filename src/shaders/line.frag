#version 330 core
out vec4 FragColor;

uniform vec3 u_line_color;

void main() {
    FragColor = u_line_color;
}