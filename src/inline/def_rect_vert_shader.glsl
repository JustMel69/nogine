#version 330 core

layout (location = 0) in vec2 v_Pos;
layout (location = 1) in vec4 v_Col;

out vec4 f_Col;

uniform mat3 mvm;

void main() {
    gl_Position = vec4(mvm * vec3(v_Pos, 1.0), 1.0);
    f_Col = v_Col;
}