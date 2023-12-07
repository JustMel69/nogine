#version 420 core

layout (location = 0) out vec4 o_Col;

in vec2 f_Uv;
in float f_Alpha;

layout (binding = 0) uniform sampler2D screen_tex;

void main() {
    o_Col = texture(screen_tex, f_Uv) * vec4(1.0, 1.0, 1.0, f_Alpha);
}