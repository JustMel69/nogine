use std::time::Instant;

use nogine::{graphics::{buffers::{GlBuffer, GlVAO}, shader::{SubShader, SubShaderType, Shader}, verts::set_vertex_attribs, Graphics}, window::WindowCfg, color::{Color4, Color}, math::Vector2};

const VERTEX_SHADER_SOURCE: &str = "#version 330 core\n
layout (location = 0) in vec2 aPos;\n
layout (location = 1) in vec4 aCol;\n

out vec4 vCol;

void main()\n
{\n
    gl_Position = vec4(aPos.x, aPos.y, 0.0, 1.0);\n
    vCol = aCol;
}";

const FRAG_SHADER_SRC: &str = "#version 330 core\n
in vec4 vCol;

out vec4 FragColor;\n
void main()\n
{\n
    FragColor = vCol;\n
}";

fn main() { unsafe {
    let mut window = WindowCfg::default().main(true).res((800, 600)).title("LearnOpenGL").mode(nogine::window::WindowMode::Windowed).init().unwrap();    

    let vert_subshader = SubShader::new(VERTEX_SHADER_SOURCE, SubShaderType::Vert);
    let frag_subshader = SubShader::new(FRAG_SHADER_SRC, SubShaderType::Frag);
    let shader = Shader::new(vert_subshader, frag_subshader);
/*
    #[repr(C)]
    struct Vert(Vector2, Color4);

    let vert_data = [Vert(Vector2(-0.5f32, -0.5), Color4(0.0, 0.0, 0.0, 1.0)), Vert(Vector2(-0.5, 0.5), Color4(0.0, 1.0, 0.0, 1.0)), Vert(Vector2(0.5, 0.5), Color4(1.0, 1.0, 0.0, 1.0)), Vert(Vector2(0.5, -0.5), Color4(1.0, 0.0, 0.0, 1.0))];
    let tri_data = [0u32, 1, 2, 2, 3, 0];

    
    let vao = GlVAO::new();

    vao.bind();    

    let vbo = GlBuffer::new(gl::ARRAY_BUFFER);
    vbo.set_data(&vert_data);

    let ebo = GlBuffer::new(gl::ELEMENT_ARRAY_BUFFER);
    ebo.set_data(&tri_data);

    set_vertex_attribs(&[2, 4]); */

    let mut prev_frame = Instant::now();
    let mut ts = 1.0 / 60.0;
    let mut time = 0.0;
    while window.is_running() {
        Graphics::tick();
        window.clear_screen(Color4(0.2, 0.3, 0.3, 1.0));
        
        shader.enable();
        //vao.bind();
        //gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());

        time += ts;        
        Graphics::draw_rect_full(Vector2(-0.5, -0.5), Vector2::ONE, time, [Color4::BLUE, Color4::GREEN, Color4::YELLOW, Color4::RED]);
        
        window.swap_buffers();
        window.handle_events();
        window.force_framerate(prev_frame, 60.0);
        ts = prev_frame.elapsed().as_secs_f32();
        prev_frame = Instant::now();
    }
}}