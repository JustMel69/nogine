#version 330 core

layout (location = 0) out vec4 o_Col;

in vec4 f_Col;
in vec2 f_UV;

uniform sampler2D main_tex;

void main() {
    o_Col = texture(main_tex, f_UV) * f_Col;
    //o_Col = vec4(f_UV, 0.0, 1.0);
}