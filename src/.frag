#version 330 core

in vec3 texCoord;
in vec3 Position_worldspace;
in vec3 Normal_cameraspace;
in vec3 LightDirection_cameraspace;
in vec3 EyeDirection_cameraspace;
in vec3 fill;

out vec3 Color;

uniform float u_time;
uniform vec3 u_sun_dir_vec3;
uniform sampler2D texture0;
uniform vec3 u_atmos_color;
uniform vec3 u_emissive_color;

void main()
{
    vec2 idx = vec2(int(texCoord.x * 24.0) / 24.0, int(texCoord.y * 24.0) / 24.0);
    vec3 MaterialDiffuseColor = texture(texture0, idx).xyz;

    if (length(Position_worldspace) >= 10000.0) {
        Color = MaterialDiffuseColor; // If it's so far away, just shade it all one color
        return;
    }

    vec3 LightColor = vec3(1.0, 1.0, 1.0);
    float LightPower = 1.0;

    // Normal of the computed fragment, in camera space
	vec3 n = normalize( Normal_cameraspace );
    vec3 l = normalize( LightDirection_cameraspace );
    vec3 e = normalize( EyeDirection_cameraspace );
    
	// Distance to the light
	float distance = length( u_sun_dir_vec3 - Position_worldspace );

    float cosTheta = clamp(dot( n,l ), 0, 1);
    float cosTheta2 = clamp( 0.5 * dot( n,l ) + 0.2, 0,1 );
    float a = dot(n, e);
    float atmosphere = clamp(pow(a-1,4), 0, 1);

    Color = 
        // Emissive color - ie for the Sun
        + u_emissive_color
        // Diffuse light
        + MaterialDiffuseColor * LightColor * LightPower * (cosTheta + 0.0000005 * distance) / max(0.000002 * distance, 1.0)
        // Atmosphere
        + atmosphere * LightColor * LightPower * cosTheta2 * u_atmos_color;
}