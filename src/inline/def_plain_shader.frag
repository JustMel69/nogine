#version 330 core

layout (location = 0) out vec4 o_Col;

in vec4 f_Col;

void main() {
    o_Col = f_Col;
}