#version 330 core

in vec3 Position_worldspace;
in vec3 Normal_cameraspace;
in vec3 LightDirection_cameraspace;

out vec3 Color;

uniform float u_time;
uniform vec3 u_sun_pos_vec3;

void main()
{
    vec3 MaterialDiffuseColor = vec3(0.8, 0.6, 0.2);
    vec3 LightColor = vec3(1.0, 1.0, 1.0);
    float LightPower = 1e9;

    // Normal of the computed fragment, in camera space
	vec3 n = normalize( Normal_cameraspace );
    vec3 l = normalize( LightDirection_cameraspace );
    
	// Distance to the light
	float distance = length( u_sun_pos_vec3 - Position_worldspace );

    float cosTheta = clamp( dot( n,l ), 0,1 );

    Color = vec3(0, 0, 0.0783) + MaterialDiffuseColor * LightColor * LightPower * cosTheta / (distance * distance);
}