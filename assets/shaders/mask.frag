#version 450


/* in vec2 fragTexCoord; */
layout(location = 0) in vec2 v_Uv;
/* in vec3 fragNormal; */
layout(location = 1) in vec3 fragNormal;
/* in vec3 fragPosition; */
layout(location = 2) in vec3 fragPosition;

layout(location = 0) out vec4 finalColor;

layout(set = 1, binding = 0) uniform CustomMaterial {
    vec4 Color;
};

layout(set = 1, binding = 1) uniform texture2D CustomMaterial_texture;
layout(set = 1, binding = 2) uniform sampler CustomMaterial_sampler;


vec4 colDiffuse = vec4(1.0, 1.0, 1.0, 1.0);

vec3 alumi[4] = vec3[]( vec3(0.223, 0.192, 0.294), vec3(0.490, 0.439, 0.443), vec3(0.627, 0.576, 0.556), vec3(0.874, 0.964, 0.960));
vec3 steel[4] = vec3[]( vec3(0.223, 0.192, 0.294), vec3(0.188, 0.172, 0.180), vec3(0.352, 0.325, 0.325), vec3(0.490, 0.439, 0.443));
vec3 tire[4] = vec3[]( vec3(0.223, 0.192, 0.294), vec3(0.188, 0.172, 0.180), vec3(0.223, 0.278, 0.470), vec3(0.352, 0.325, 0.325));
vec3 wood[4] = vec3[]( vec3(0.223, 0.192, 0.294), vec3(0.627, 0.356, 0.325), vec3(0.749, 0.474, 0.345), vec3(0.933, 0.631, 0.376));
vec3 grass[4] = vec3[]( vec3(0.223, 0.192, 0.294), vec3(0.223, 0.482, 0.266), vec3(0.443, 0.666, 0.203), vec3(0.713, 0.835, 0.235));
vec3 leaf[4] = vec3[]( vec3(0.223, 0.192, 0.294), vec3(0.235, 0.349, 0.337), vec3(0.223, 0.482, 0.266), vec3(0.443, 0.666, 0.203));
vec3 sand[4] = vec3[]( vec3(0.223, 0.192, 0.294), vec3(0.956, 0.705, 0.105), vec3(0.933, 0.631, 0.376), vec3(0.956, 0.8, 0.631));
vec3 sea[4] = vec3[]( vec3(0.223, 0.192, 0.294), vec3(0.223, 0.278, 0.470), vec3(0.156, 0.8, 0.874), vec3(0.874, 0.964, 0.960));
vec3 sky[4] = vec3[]( vec3(0.223, 0.470, 0.658), vec3(0.156, 0.8, 0.874), vec3(0.541, 0.921, 0.945), vec3(0.874, 0.964, 0.960));
vec3 warn[4] = vec3[]( vec3(0.223, 0.192, 0.294), vec3(0.956, 0.494, 0.105), vec3(0.956, 0.494, 0.105), vec3(0.956, 0.705, 0.105));
vec3 blood[4] = vec3[]( vec3(0.223, 0.192, 0.294), vec3(0.662, 0.231, 0.231), vec3(0.901, 0.282, 0.180), vec3(0.956, 0.494, 0.105));
vec3 clear[4] = vec3[]( vec3(0.223, 0.192, 0.294), vec3(0.811, 0.776, 0.721), vec3(0.874, 0.964, 0.960), vec3(0.874, 0.964, 0.960));

float luma(vec3 color) {
  return dot(color, vec3(0.299, 0.587, 0.114));
}
float luma(vec4 color) {
  return dot(color.rgb, vec3(0.299, 0.587, 0.114));
}

vec3 dithermono(vec2 pos, vec4 col, vec4 maskCol) {
    int red = int(maskCol.r * 256.0);
    int green = int(maskCol.g * 256.0);
    int blue = int(maskCol.b * 256.0);
    vec3 palette[4] = tire;


    if (red < 64) { // 20
        if (green < 64) { // 20
            if (blue < 64) { // 20
                palette = tire;
            }; 
        } else if (green < 128) { // 60
            if (blue < 64) { // 20
                palette = leaf;
            } else { // E0
                palette = sea;
            };
        } else if (green < 192) { // A0
            if (blue < 64) { // 20
            } else { // E0
                palette = sky;
            };
        };
    } else if (red < 128) { // 60
        if (green < 64) { // 20
            if (blue < 64) { // 20
                palette = wood;
            };
        } else if (green < 128) { // 60
            if (blue < 64) { // 20
            } else if (blue < 128) { // 60
                palette = steel;
            };
        } else if (green < 192) { // A0
            if (blue < 64) { // 20
                palette = grass;
            };
        } else { // E0
            if (blue < 64) { // 20
            };
        };
    } else if (red < 192) { // A0
        if (green < 64) { // 20
            if (blue < 64) { // 20
            };
        } else if (green < 128) { // 60
            if (blue < 64) { // 20
            };
        } else if (green < 192) { // A0
            if (blue < 64) { // 20
                palette = alumi;
            };
        } else { // E0
            if (blue < 64) { // 20
            };
        };
    } else { // E0
        if (green < 64) { // 20
            if (blue < 64) { // 20
                palette = blood;
            };
        } else if (green < 128) { // 60
            if (blue < 64) { // 20
            };
        } else if (green < 192) { // A0
            if (blue < 64) { // 20
                palette = warn;
            };
        } else { // E0
            /* if (blue < 64) { // 20 */
            if (blue < 192) {
                palette = sand;
            } else { // E0
                palette = clear;
            };
        };
    };

    float bands = 4;
    float bri = luma(col);

    int pixelSize = 1;
    pos = pos - mod(pos, pixelSize);
	float x = floor(mod(pos.x, 4.0*pixelSize))/pixelSize;
	float y = floor(mod(pos.y, 4.0*pixelSize))/pixelSize;
	float index = floor(x + y * 4.0);

	float limit = 0.0;
	float stepp = 1.0 / bands;
	/* float stepp = 2.0 / bands; */

    float matrix[16] = float[](
		0.0625, 0.5625, 0.1875, 0.6875, 
        0.8125, 0.3125, 0.9375, 0.4375, 
        0.25,   0.75,   0.125,  0.625,  
        1.0,    0.5,    0.875,  0.375   
    );

    limit = matrix[int(index)];
	float a = bri - mod(bri,stepp);
    float b = a + stepp;
	limit = limit/bands + a;
	float _out = a;
	if (bri > limit) { _out = b; };
    return palette[int(floor(_out*bands*0.99))];
}

void main() {
    vec4 texelColor = vec4(0.5, 0.5, 0.5, 1.0);
    vec3 lightDot = vec3(0.0);
    vec3 normal = normalize(fragNormal);
    vec3 specular = vec3(0.0);
    vec3 light = vec3(0.0);

    light = -normalize(vec3(0.0,0.0,0.0) - vec3(1.0,1.0,1.0));
    float NdotL = max(dot(normal, light), 0.0);
    lightDot += vec3(1.0,1.0,1.0).rgb*NdotL;

    float specCo = 0.0;
    // Specularity
    specular += specCo;

    finalColor += texelColor*vec4(0.4,0.4,0.4,1.0)/10.0;
    finalColor = (texelColor*((colDiffuse + vec4(specular, 1.0))*0.5*vec4(lightDot, 1.0)));

    // Gamma correction
    finalColor = pow(finalColor, vec4(1.0/2.2));

    vec4 maskColor = texture(sampler2D(CustomMaterial_texture,CustomMaterial_sampler), v_Uv);
    maskColor = pow(maskColor, vec4(1.0/2.2));
    finalColor = vec4(dithermono(gl_FragCoord.xy, finalColor, maskColor), 1.0);
    finalColor = pow(finalColor, vec4(2.2/1.0));
}
