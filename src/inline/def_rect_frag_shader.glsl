#version 330 core

layout (location = 0) out vec4 outCol;

in vec4 fCol;

void main() {
    outCol = fCol;
}