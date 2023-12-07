use nogine::{graphics::{Graphics, pipeline::{RenderPipeline, RenderTexture, SceneRenderData}, RenderStats, texture::TextureFiltering, BlendingMode}, window::{WindowCfg, WindowMode}, color::{Color4, Color}, math::Vector2, unwrap_res};

struct CustomPipeline {
    intensity: f32,
    iterations: u32,
}

impl RenderPipeline for CustomPipeline {
    fn render(&self, screen_rt: &mut RenderTexture, scene_data: &SceneRenderData, stats: &mut RenderStats) {
        // Render scene to texture
        let mut src_rt = RenderTexture::sized_as(&screen_rt, TextureFiltering::Closest);
        src_rt.clear(scene_data.clear_col());
        src_rt.render_scene(scene_data, stats);

        // Apply effect
        naive_blur(&src_rt, screen_rt, self.iterations, self.intensity, stats);
    }
}

fn naive_blur(source: &RenderTexture, target: &mut RenderTexture, iterations: u32, intensity: f32, stats: &mut RenderStats) {
    target.clear(Color4::CLEAR);
    target.combine(&source, BlendingMode::AlphaMix, stats);

    for i in 1..=iterations {
        let mut downscaled_rt = source.downscaled(5 * i, TextureFiltering::Linear, stats);
        downscaled_rt.set_alpha(intensity / i as f32);

        target.combine(&downscaled_rt, BlendingMode::AlphaMix, stats);
    }
}

fn main() {
    // Create Window
    let mut window = unwrap_res!(WindowCfg::default().main(true).res((1280, 720)).title("Naive Blur Example").mode(WindowMode::Windowed).init());

    // Setup graphics
    Graphics::set_cam(Vector2::ZERO, 1.5);
    Graphics::set_clear_col(Color4(0.3, 0.2, 0.1, 1.0));

    let pipeline = CustomPipeline {
        intensity: 0.75,
        iterations: 25,
    };
    
    while window.is_running() {
        window.pre_tick(Some(&pipeline));
        
        Graphics::draw_rect(Vector2(-1.55, -0.75), Vector2::ONE, Color4::CYAN);
        Graphics::draw_circle(Vector2(0.0, 0.25), 0.5, Color4::YELLOW);
        Graphics::draw_polygon(Vector2(1.0, -0.25), 0.5, 0.0, 5, Color::PINK);
        
        window.post_tick();
    }
}