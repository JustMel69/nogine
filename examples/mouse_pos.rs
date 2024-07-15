use nogine::{color::{Color, Color4}, graphics::{gfx::{self, integer_scaling}, pipeline::{RenderPipeline, RenderTexture, SceneRenderData, DEFAULT_RENDER_TARGET}, texture::TextureFiltering, Graphics, RenderStats}, input::{Input, MouseInput}, log_info, math::{uvec2, vec2}, unwrap_res, window::{WindowCfg, WindowMode}};

struct CustomPipeline;

impl RenderPipeline for CustomPipeline {
    fn render(&self, screen_rt: &mut RenderTexture, scene_data: &SceneRenderData, _ui_data: Option<&SceneRenderData>, stats: &mut RenderStats) {
        // Render scene to texture
        let mut src_rt = RenderTexture::new(uvec2(240, 160), TextureFiltering::Closest);
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
    
    Graphics::set_cam(vec2::ZERO, vec2(1.5 * 5.0, 5.0));

    let mut click_pos = vec2::ZERO;
    while window.is_running() {
        //window.set_aspect_ratio(3, 2);
        window.pre_tick(Some(&pipeline));

        if Input::mouse_pressed(MouseInput::Left) {
            let src_pos = Input::mouse_pos();
            log_info!("Src: {src_pos}");
            if let Ok(x) = gfx::integer_scaling_mouse_pos(src_pos, uvec2(240, 160), window.get_size()) {
                log_info!("Processed: {x}");
                click_pos = gfx::screen_to_world_pos(x, uvec2(240, 160));
            }
        }

        Graphics::draw_circle(click_pos, 0.5, Color4::RED);
        
        window.post_tick();
    }
}