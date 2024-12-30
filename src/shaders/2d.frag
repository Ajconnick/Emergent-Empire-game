#version 330 core

uniform float u_opacity;
uniform sampler2D texture0;

uniform vec2 u_sprite_size;   // size of a single sprite in pixels
uniform vec2 u_sprite_offset; // offset in the spritesheet (in pixels)

in vec3 uv;

out vec4 Color;

void main()
{
    Color = texture(texture0, uv.xy * u_sprite_size + u_sprite_offset);
}