#version 450

layout(location = 0) in uint data;
layout(location = 0) out vec3 fragColor;
layout(push_constant) uniform push_constants {
    mat4 chunkTransform;
};

vec3 colors[6] = vec3[](
vec3(1.0, 0.0, 0.0),
vec3(1.0, 0.5, 0.0),
vec3(1.0, 1.0, 0.0),
vec3(0.0, 1.0, 0.0),
vec3(0.0, 0.0, 1.0),
vec3(1.0, 0.0, 1.0)
);

float cornerIndicesI[6] = float[](-0.5, -0.5,  0.5, 0.5,  0.5, -0.5);
float cornerIndicesJ[6] = float[](-0.5,  0.5, -0.5, 0.5, -0.5,  0.5);

mat4 faceTransforms[6] = mat4[](
mat4(0, 1, 0, 0, 0, 0, 1, 0,  1, 0, 0, 0, 0, 0, 0, 1), // +x
mat4(0, 0, 1, 0, 0, 1, 0, 0, -1, 0, 0, 0, 0, 0, 0, 1), // -x
mat4(0, 0, 1, 0, 1, 0, 0, 0, 0,  1, 0, 0, 0, 0, 0, 1), // +y
mat4(1, 0, 0, 0, 0, 0, 1, 0, 0, -1, 0, 0, 0, 0, 0, 1), // -y
mat4(1, 0, 0, 0, 0, 1, 0, 0, 0, 0,  1, 0, 0, 0, 0, 1), // +z
mat4(0, 1, 0, 0, 1, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 1)  // -z
);

void main() {
    uint direction = bitfieldExtract(data, 6, 3);

    vec4 chunkCubeCoord = vec4(bitfieldExtract(data, 0, 2), bitfieldExtract(data, 2, 2), bitfieldExtract(data, 4, 2), 0);
    vec4 faceVertexCoord = vec4(cornerIndicesI[gl_VertexIndex], cornerIndicesJ[gl_VertexIndex], 0.5, 1);
    vec4 cubeVertexCoord = faceTransforms[direction] * faceVertexCoord;
    vec4 vertexChunkCoord = chunkCubeCoord + cubeVertexCoord;

    gl_Position = chunkTransform * vertexChunkCoord;
    fragColor = colors[direction];
}
