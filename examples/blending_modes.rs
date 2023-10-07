use nogine::{graphics::{Graphics, BlendingMode}, window::WindowCfg, color::{Color4, Color}, math::Vector2};

fn main() {
    // Create Window
    let mut window = WindowCfg::default().main(true).res((1280, 720)).title("Blending Modes Example").mode(nogine::window::WindowMode::Windowed).init().expect("Couldn't open window");

    // Setup graphics
    Graphics::set_cam(Vector2::ZERO, 1.5);
    while window.is_running() {
        // Refresh graphics
        Graphics::tick(window.aspect_ratio());
        window.clear_screen(Color4(0.1, 0.2, 0.3, 1.0));
        
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

        // Handle window
        window.swap_buffers();
        window.handle_events();
    }
}