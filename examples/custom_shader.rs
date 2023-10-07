use nogine::{graphics::{Graphics, shader::{Shader, SubShader, SubShaderType}, DefaultShaders, Mode}, window::WindowCfg, color::{Color4, Color}, math::Vector2};

const CUSTOM_VERT: &str = r#"
#version 330 core

layout (location = 0) in vec2 v_Pos;
layout (location = 1) in vec4 v_Col;

out vec4 f_Col;

uniform mat3 mvm;

void main() {
    gl_Position = vec4(mvm * vec3(v_Pos, 1.0), 1.0);
    f_Col = vec4(v_Pos, 1.0, 1.0);
}
"#;

fn main() {
    // Create Window
    let mut window = WindowCfg::default().main(true).res((1280, 720)).title("Custom Shader Example").mode(nogine::window::WindowMode::Windowed).init().expect("Couldn't open window");
    
    // Create shader
    let shader = Shader::new(&SubShader::new(&CUSTOM_VERT, SubShaderType::Vert), &DefaultShaders::def_plain_frag());

    // Setup graphics
    Graphics::set_cam(Vector2::ZERO, 1.5);
    Graphics::set_shader(Some(shader), Mode::Rect);

    while window.is_running() {
        // Refresh graphics
        Graphics::tick(window.aspect_ratio());
        window.clear_screen(Color4::BLACK);
        
        Graphics::draw_rect(Vector2(-0.5, -0.5), Vector2::ONE, Color4::WHITE);
        
        // Handle window
        window.swap_buffers();
        window.handle_events();
    }
}