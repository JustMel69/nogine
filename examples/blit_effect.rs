use nogine::{graphics::{Graphics, pipeline::{RenderPipeline, RenderTexture, SceneRenderData, DEFAULT_RENDER_TARGET}, RenderStats, shader::Shader, texture::TextureFiltering, BlendingMode, material::Material}, window::{WindowCfg, WindowMode}, color::{Color4, Color}, math::Vector2, unwrap_res};

struct CustomPipeline {
    material: Material
}

const SHADER_SRC: &str = include_str!("res/invert_blit.frag");

impl RenderPipeline for CustomPipeline {
    fn render(&self, screen_rt: &mut RenderTexture, scene_data: &SceneRenderData, stats: &mut RenderStats) {
        // Render scene to texture
        let mut src_rt = RenderTexture::sized_as(&screen_rt, TextureFiltering::Closest);
        src_rt.clear(scene_data.clear_col());
        src_rt.render_scene(scene_data, DEFAULT_RENDER_TARGET, stats);
        
        // Render texture to screen with a custom material
        screen_rt.render_with_shader(&src_rt, &self.material, BlendingMode::AlphaMix, stats);
    }
}

fn main() {
    // Create Window
    let mut window = unwrap_res!(WindowCfg::default().main(true).res((1280, 720)).title("Blit Example").mode(WindowMode::Windowed).init());

    // Setup graphics
    Graphics::set_clear_col(Color4(0.3, 0.2, 0.1, 1.0));

    let shader = unwrap_res!(Shader::new_blit(SHADER_SRC));
    let material = Material::new(&shader, &[]);

    let pipeline = CustomPipeline { material };
    
    while window.is_running() {
        window.pre_tick(Some(&pipeline));
        
        Graphics::set_cam(Vector2::ZERO, Vector2(1.5 * window.aspect_ratio(), 1.5));

        Graphics::draw_rect(Vector2(-1.55, -0.75), Vector2::ONE, Color4::CYAN);
        Graphics::draw_circle(Vector2(0.0, 0.25), 0.5, Color4::YELLOW);
        Graphics::draw_polygon(Vector2(1.0, -0.25), 0.5, 0.0, 5, Color::PINK);
        
        window.post_tick();
    }
}