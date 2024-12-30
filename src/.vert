#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Normal_modelspace;
layout (location = 2) in vec3 texture_coord;

uniform vec2 u_resolution;
uniform vec3 u_sun_dir_vec3;
uniform mat4 u_model_matrix;
uniform mat4 u_view_matrix;
uniform mat4 u_proj_matrix;

out vec3 texCoord;
out vec3 Position_worldspace;
out vec3 Position_viewspace;
out vec3 Normal_cameraspace;
out vec3 LightDirection_cameraspace;
out vec3 EyeDirection_cameraspace;
out vec3 fill;

void main()
{
    vec4 mod_pos = u_model_matrix * vec4(Position, 1.0);
    float d = 10000.0;
    if (length(mod_pos.xyz) >= d) {
        mod_pos = vec4(normalize(mod_pos.xyz) * d, 1.0);
    }
    vec4 view_pos = u_view_matrix * mod_pos;
    vec4 uv = u_proj_matrix * view_pos;

    Position_worldspace = mod_pos.xyz;
    Position_worldspace = view_pos.xyz;

    if (u_resolution.x > u_resolution.y) {
        uv.x *= u_resolution.y / u_resolution.x;
    } else {
        uv.y *= u_resolution.x / u_resolution.y;
    }

	Normal_cameraspace = ( u_view_matrix * u_model_matrix * vec4(Normal_modelspace,0)).xyz; // Only correct if ModelMatrix does not scale the model ! Use its inverse transpose if not.
    
    vec3 vertexPosition_cameraspace = view_pos.xyz;
	EyeDirection_cameraspace = vec3(0,0,0) - vertexPosition_cameraspace;

    vec3 LightPosition_cameraspace = ( u_view_matrix * vec4(u_sun_dir_vec3, 1.0)).xyz;
	LightDirection_cameraspace = LightPosition_cameraspace + EyeDirection_cameraspace;

    gl_Position = uv;
    texCoord = texture_coord;
}