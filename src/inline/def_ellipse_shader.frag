#version 330 core

layout (location = 0) out vec4 o_Col;

in vec4 f_Col;
in vec2 f_UV;

void main() {
    if (distance(f_UV, vec2(0.5)) > 0.5) {
        discard;
    }
    
    o_Col = f_Col;
}