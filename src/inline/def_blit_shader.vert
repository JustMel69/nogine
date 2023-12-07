#version 330 core

layout (location = 0) in vec2 v_Pos;
layout (location = 1) in vec2 v_Uv;
layout (location = 2) in float v_Alpha;

out vec2 f_Uv;
out float f_Alpha;

void main() {
    gl_Position = vec4(v_Pos, 0.0, 1.0);
    f_Uv = v_Uv;
    f_Alpha = v_Alpha;
}