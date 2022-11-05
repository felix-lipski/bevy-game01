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


float luma(vec3 color) {
  return dot(color, vec3(0.299, 0.587, 0.114));
}
float luma(vec4 color) {
  return dot(color.rgb, vec3(0.299, 0.587, 0.114));
}

float linear_to_s (float theLinearValue) {
  if (theLinearValue <= 0.0031308f) {
    return theLinearValue * 12.92f;
  }
  return pow(theLinearValue, 1.0f/2.4f) * 1.055f - 0.055f;
}

float s_to_linear (float thesRGBValue) {
  if (thesRGBValue <= 0.04045f) {
    return thesRGBValue / 12.92f;
  }
  return pow((thesRGBValue + 0.055f) / 1.055f, 2.4f);
}


vec4 colDiffuse = vec4(1.0, 1.0, 1.0, 1.0);


vec3 alumi[4]    = vec3[](vec3(0.07031, 0.07031, 0.07031),vec3(0.48828, 0.43750, 0.44140),vec3(0.62500, 0.57421, 0.55468),vec3(0.87109, 0.96093, 0.95703));
vec3 steel[4]    = vec3[](vec3(0.07031, 0.07031, 0.07031),vec3(0.18750, 0.17187, 0.17968),vec3(0.35156, 0.32421, 0.32421),vec3(0.48828, 0.43750, 0.44140));
vec3 tire[4]     = vec3[](vec3(0.07031, 0.07031, 0.07031),vec3(0.18750, 0.17187, 0.17968),vec3(0.22265, 0.27734, 0.46875),vec3(0.35156, 0.32421, 0.32421));
vec3 wood[4]     = vec3[](vec3(0.07031, 0.07031, 0.07031),vec3(0.62500, 0.35546, 0.32421),vec3(0.74609, 0.47265, 0.34375),vec3(0.92968, 0.62890, 0.37500));
vec3 grass[4]    = vec3[](vec3(0.07031, 0.07031, 0.07031),vec3(0.22265, 0.48046, 0.26562),vec3(0.44140, 0.66406, 0.20312),vec3(0.71093, 0.83203, 0.23437));
vec3 leaf[4]     = vec3[](vec3(0.07031, 0.07031, 0.07031),vec3(0.23437, 0.34765, 0.33593),vec3(0.22265, 0.48046, 0.26562),vec3(0.44140, 0.66406, 0.20312));
vec3 sand[4]     = vec3[](vec3(0.07031, 0.07031, 0.07031),vec3(0.95312, 0.70312, 0.10546),vec3(0.92968, 0.62890, 0.37500),vec3(0.95312, 0.79687, 0.62890));
vec3 sea[4]      = vec3[](vec3(0.07031, 0.07031, 0.07031),vec3(0.22265, 0.27734, 0.46875),vec3(0.15625, 0.79687, 0.87109),vec3(0.87109, 0.96093, 0.95703));
vec3 sky[4]      = vec3[](vec3(0.07031, 0.07031, 0.07031),vec3(0.22265, 0.46875, 0.65625),vec3(0.15625, 0.79687, 0.87109),vec3(0.53906, 0.91796, 0.94140));
vec3 warn[4]     = vec3[](vec3(0.07031, 0.07031, 0.07031),vec3(0.95312, 0.49218, 0.10546),vec3(0.95312, 0.49218, 0.10546),vec3(0.95312, 0.70312, 0.10546));
vec3 blood[4]    = vec3[](vec3(0.07031, 0.07031, 0.07031),vec3(0.66015, 0.23046, 0.23046),vec3(0.89843, 0.28125, 0.17968),vec3(0.95312, 0.49218, 0.10546));
vec3 clear[4]    = vec3[](vec3(0.07031, 0.07031, 0.07031),vec3(0.80859, 0.77343, 0.71875),vec3(0.87109, 0.96093, 0.95703),vec3(0.87109, 0.96093, 0.95703));


float dither_mask(float limit, vec4 in_mono, float bands) {
	float stepp = 1.0 / bands;
    float bri = luma(in_mono);
	float a = bri - mod(bri,stepp);
    float b = a + stepp;
	limit = limit/bands + a;
	float _out = a;
	if (bri > limit) { _out = b; };
    return _out;
}

int dithered_palette_index(float limit, vec4 in_mono, float bands) {
    float f = dither_mask(limit, in_mono, bands);
    return int(floor(f * bands * 0.99));
}

vec3 alumi_f  (float limit, vec4 in_mono) { int idx = dithered_palette_index(limit, in_mono, 4); return alumi[idx] ; } 
vec3 steel_f  (float limit, vec4 in_mono) { int idx = dithered_palette_index(limit, in_mono, 4); return steel[idx] ; } 
vec3 tire_f   (float limit, vec4 in_mono) { int idx = dithered_palette_index(limit, in_mono, 4); return tire[idx]  ; } 
vec3 wood_f   (float limit, vec4 in_mono) { int idx = dithered_palette_index(limit, in_mono, 4); return wood[idx]  ; } 
vec3 grass_f  (float limit, vec4 in_mono) { int idx = dithered_palette_index(limit, in_mono, 4); return grass[idx] ; } 
vec3 leaf_f   (float limit, vec4 in_mono) { int idx = dithered_palette_index(limit, in_mono, 4); return leaf[idx]  ; } 
vec3 sand_f   (float limit, vec4 in_mono) { int idx = dithered_palette_index(limit, in_mono, 4); return sand[idx]  ; } 
vec3 sea_f    (float limit, vec4 in_mono) { int idx = dithered_palette_index(limit, in_mono, 4); return sea[idx]   ; } 
vec3 sky_f    (float limit, vec4 in_mono) { int idx = dithered_palette_index(limit, in_mono, 4); return sky[idx]   ; } 
vec3 warn_f   (float limit, vec4 in_mono) { int idx = dithered_palette_index(limit, in_mono, 4); return warn[idx]  ; } 
vec3 blood_f  (float limit, vec4 in_mono) { int idx = dithered_palette_index(limit, in_mono, 4); return blood[idx] ; } 
vec3 clear_f  (float limit, vec4 in_mono) { int idx = dithered_palette_index(limit, in_mono, 4); return clear[idx] ; } 

vec3 apply_material(vec4 in_mat, float limit, vec4 in_mono) {
    int red   = int(in_mat.r * 256.0);
    int green = int(in_mat.g * 256.0);
    int blue  = int(in_mat.b * 256.0);

    if (red < 64) { // 20
        if (green < 64) { // 20
            if (blue < 64) { // 20
                return tire_f(limit, in_mono);
            }; 
        } else if (green < 128) { // 60
            if (blue < 64) { // 20
                return leaf_f(limit, in_mono);
            } else { // E0
                return sea_f(limit, in_mono);
            };
        } else if (green < 192) { // A0
            if (blue < 64) { // 20
            } else { // E0
                return sky_f(limit, in_mono);
            };
        };
    } else if (red < 128) { // 60
        if (green < 64) { // 20
            if (blue < 64) { // 20
                return wood_f(limit, in_mono);
            };
        } else if (green < 128) { // 60
            if (blue < 64) { // 20
            } else if (blue < 128) { // 60
                return steel_f(limit, in_mono);
            };
        } else if (green < 192) { // A0
            if (blue < 64) { // 20
                return grass_f(limit, in_mono);
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
                return alumi_f(limit, in_mono);
            };
        } else { // E0
            if (blue < 64) { // 20
            };
        };
    } else { // E0
        if (green < 64) { // 20
            if (blue < 64) { // 20
                return blood_f(limit, in_mono);
            };
        } else if (green < 128) { // 60
            if (blue < 64) { // 20
            };
        } else if (green < 192) { // A0
            if (blue < 64) { // 20
                return warn_f(limit, in_mono);
            };
        } else { // E0
            /* if (blue < 64) { // 20 */
            if (blue < 192) {
                return sand_f(limit, in_mono);
            } else { // E0
                return clear_f(limit, in_mono);
            };
        };
    };
    return clear_f(limit, in_mono);

}


float dither_matrix_lookup(vec2 pos) {
    int pixelSize = 1;
    pos = pos - mod(pos, pixelSize);
	float x = floor(mod(pos.x, 4.0*pixelSize))/pixelSize;
	float y = floor(mod(pos.y, 4.0*pixelSize))/pixelSize;
	float index = floor(x + y * 4.0);
    float matrix[16] = float[](
		0.0625, 0.5625, 0.1875, 0.6875, 
        0.8125, 0.3125, 0.9375, 0.4375, 
        0.25,   0.75,   0.125,  0.625,  
        1.0,    0.5,    0.875,  0.375   
    );
    return matrix[int(index)];
}

vec3 dithermono(vec2 pos, vec4 in_mono, vec4 in_mat) {
    float limit = dither_matrix_lookup(pos);

    return apply_material(in_mat, limit, in_mono);
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

    float specCo = 3.0;
    // Specularity
    specular += specCo;

    finalColor += texelColor*vec4(0.4,0.4,0.4,1.0)/10.0;
    finalColor = (texelColor*((colDiffuse + vec4(specular, 1.0))*0.5*vec4(lightDot, 1.0)));

    // Gamma correction
    finalColor = pow(finalColor, vec4(1.0/2.2));

    vec4 maskColor = texture(sampler2D(CustomMaterial_texture,CustomMaterial_sampler), v_Uv);
    maskColor = pow(maskColor, vec4(1.0/2.2));
    finalColor = vec4(dithermono(gl_FragCoord.xy, finalColor, maskColor), 1.0);
    finalColor = vec4( vec3(s_to_linear(finalColor.r), s_to_linear(finalColor.g), s_to_linear(finalColor.b)), 1.0 );
    /* finalColor = pow(finalColor, vec4(2.2/1.0)); */
}
