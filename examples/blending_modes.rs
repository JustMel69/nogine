use nogine::{graphics::{Graphics, BlendingMode}, window::{WindowCfg, WindowMode}, color::{Color4, Color}, math::Vector2, unwrap_res};

fn main() {
    // Create Window
    let mut window = unwrap_res!(WindowCfg::default().main(true).res((1280, 720)).title("Blending Modes Example").mode(WindowMode::Windowed).init());

    // Setup graphics
    Graphics::set_clear_col(Color4(0.1, 0.2, 0.3, 1.0));

    while window.is_running() {
        window.pre_tick(None);
        
        Graphics::set_cam(Vector2::ZERO, Vector2(1.5 * window.aspect_ratio(), 1.5));

        // Background
        Graphics::draw_rect(Vector2(-2.0, -0.5), Vector2(4.0, 1.0), Color4::GRAY);

        // Default is Alpha mix
        Graphics::draw_rect(Vector2(-1.75, -1.0), Vector2(1.0, 2.0), Color4::RED);

        Graphics::set_blending_mode(BlendingMode::Additive);
        Graphics::draw_rect(Vector2(-0.5, -1.0), Vector2(1.0, 2.0), Color4::GREEN);

        Graphics::set_blending_mode(BlendingMode::Multiplicative);
        Graphics::draw_rect(Vector2(0.75, -1.0), Vector2(1.0, 2.0), Color4::BLUE);

        // Restore AlphaMix once finished
        Graphics::set_blending_mode(BlendingMode::AlphaMix);

        window.post_tick();
    }
}