use nogine::{graphics::{Graphics, pipeline::{RenderPipeline, RenderTexture, SceneRenderData, DEFAULT_RENDER_TARGET}, RenderStats, texture::TextureFiltering, gfx::integer_scaling}, window::{WindowCfg, WindowMode}, color::{Color4, Color}, math::vec2, unwrap_res};

struct CustomPipeline;

impl RenderPipeline for CustomPipeline {
    fn render(&self, screen_rt: &mut RenderTexture, scene_data: &SceneRenderData, _ui_data: Option<&SceneRenderData>, stats: &mut RenderStats) {
        // Render scene to texture
        let mut src_rt = RenderTexture::new((240, 160), TextureFiltering::Closest);
        src_rt.clear(scene_data.clear_col());
        src_rt.render_scene(scene_data, DEFAULT_RENDER_TARGET, stats);
        
        // Apply integer scaling
        screen_rt.clear(Color4::BLACK);
        integer_scaling(screen_rt, &src_rt, stats);
    }
}

fn main() {
    // Create Window
    let mut window = unwrap_res!(WindowCfg::default().res((1280, 720)).title("Integer Scaling Example").mode(WindowMode::Windowed).init());

    // Setup graphics
    Graphics::set_clear_col(Color4(0.3, 0.2, 0.1, 1.0));

    let pipeline = CustomPipeline;
    
    while window.is_running() {
        //window.set_aspect_ratio(3, 2);
        window.pre_tick(Some(&pipeline));

        Graphics::set_cam(vec2::ZERO, vec2(1.5 * 1.5, 1.5));
        
        Graphics::draw_rect(vec2(-1.55, -0.75), vec2::ONE, Color4::CYAN);
        Graphics::draw_circle(vec2(0.0, 0.25), 0.5, Color4::YELLOW);
        Graphics::draw_polygon(vec2(1.0, -0.25), 0.5, 0.0, 5, Color::PINK);
        
        window.post_tick();
    }
}