#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec3 Vertex_Normal;
layout(location = 2) in vec2 Vertex_Uv;

layout(location = 0) out vec2 v_Uv;
layout(location = 1) out vec3 v_Normal;
layout(location = 2) out vec3 v_Position;

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
    mat4 View;
    mat4 InverseView;
    mat4 Projection;
    vec3 WorldPosition;
    float width;
    float height;
};

layout(set = 2, binding = 0) uniform Mesh {
    // matModel
    mat4 Model;
    // matNormal
    mat4 InverseTransposeModel;
    uint flags;
};

void main() {
    v_Uv = Vertex_Uv;
    /* v_Normal = Vertex_Normal; */
    /* v_Position = Vertex_Position; */
    gl_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);

    /* fragPosition = vec3(matModel*vec4(vertexPosition, 1.0)); */
    /* fragTexCoord = vertexTexCoord; */
    /* fragNormal = normalize(vec3(matNormal*vec4(vertexNormal, 1.0))); */
    v_Normal = normalize(vec3(InverseTransposeModel*vec4(Vertex_Normal, 1.0)));
    v_Position = vec3(Model*vec4(Vertex_Position, 1.0));
}

/* /1* #version 450 *1/ */

/* // Input vertex attributes */
/* in vec3 vertexPosition; */
/* in vec2 vertexTexCoord; */
/* in vec3 vertexNormal; */
/* in vec4 vertexColor; */

/* // Input uniform values */
/* uniform mat4 mvp; */
/* uniform mat4 matModel; */
/* uniform mat4 matNormal; */

/* // Output vertex attributes (to fragment shader) */
/* out vec3 fragPosition; */
/* out vec2 fragTexCoord; */
/* /1* out vec4 fragColor; *1/ */
/* out vec3 fragNormal; */

/* // NOTE: Add here your custom variables */

/* void main() */
/* { */
/*     // Send vertex attributes to fragment shader */
/*     fragPosition = vec3(matModel*vec4(vertexPosition, 1.0)); */
/*     fragTexCoord = vertexTexCoord; */
/*     /1* fragColor = vertexColor; *1/ */
/*     fragNormal = normalize(vec3(matNormal*vec4(vertexNormal, 1.0))); */

/*     // Calculate final vertex position */
/*     gl_Position = mvp*vec4(vertexPosition, 1.0); */
/* } */
