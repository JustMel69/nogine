use nogine::{graphics::{Graphics, shader::{Shader, SubShader, SubShaderType}, DefaultShaders, Mode, uniforms::Uniform, material::Material}, window::{WindowCfg, WindowMode}, color::{Color4, Color}, math::Vector2, unwrap_res};

const CUSTOM_VERT: &str = r#"
#version 330 core

layout (location = 0) in vec2 v_Pos;
layout (location = 1) in vec4 v_Col;

out vec4 f_Col;

uniform float blue_value;
uniform mat3 mvm;

void main() {
    gl_Position = vec4(mvm * vec3(v_Pos, 1.0), 1.0);
    f_Col = vec4(v_Pos, blue_value, 1.0);
}
"#;

fn main() {
    // Create Window
    let mut window = unwrap_res!(WindowCfg::default().res((1280, 720)).title("Custom Shader Example").mode(WindowMode::Windowed).init());
    
    // Create shader and material
    let shader = unwrap_res!(Shader::new(unwrap_res!(&SubShader::new(&CUSTOM_VERT, SubShaderType::Vert)), &DefaultShaders::def_plain_frag()));
    let material = Material::new(&shader, &[(b"blue_value\0", Uniform::Float(0.5))]);

    // Setup graphics
    Graphics::set_material(Some(material), Mode::Rect);

    while window.is_running() {
        window.pre_tick(None);

        Graphics::set_cam(Vector2::ZERO, Vector2(1.5 * window.aspect_ratio(), 1.5));
        
        Graphics::draw_rect(Vector2(-0.5, -0.5), Vector2::ONE, Color4::WHITE);
        
        window.post_tick();
    }
}