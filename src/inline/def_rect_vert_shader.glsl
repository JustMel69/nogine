#version 330 core

layout (location = 0) in vec2 vPos;
layout (location = 1) in vec4 vCol;

out vec4 fCol;

uniform mat3 mvm;

void main() {
    gl_Position = vec4(mvm * vec3(vPos, 1.0), 1.0);
    fCol = vCol;
}