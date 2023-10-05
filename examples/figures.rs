use nogine::{graphics::Graphics, window::WindowCfg, color::{Color4, Color}, math::Vector2};

fn main() {
    // Create Window
    let mut window = WindowCfg::default().main(true).res((1280, 720)).title("White Square Example").mode(nogine::window::WindowMode::Windowed).init().expect("Couldn't open window");

    // Setup graphics
    Graphics::set_cam(Vector2::ZERO, 1.5);
    while window.is_running() {
        // Refresh graphics
        Graphics::tick(window.aspect_ratio());
        window.clear_screen(Color4::BLACK);
        
        Graphics::draw_rect(Vector2(-1.5, -0.5), Vector2::ONE, Color4::WHITE);
        Graphics::draw_circle(Vector2(1.0, 0.0), 0.5, Color4::PINK);
        
        // Handle window
        window.swap_buffers();
        window.handle_events();
    }
}