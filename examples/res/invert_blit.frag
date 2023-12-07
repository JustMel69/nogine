#version 420 core

layout (location = 0) out vec4 o_Col;

in vec2 f_Uv;
in float f_Alpha;

layout (binding = 0) uniform sampler2D screen_tex;

void main() {
    vec4 col = texture(screen_tex, f_Uv);
    o_Col = vec4(mix(col.rgb, 1.0 - col.rgb, f_Uv.y), col.a * f_Alpha);
}