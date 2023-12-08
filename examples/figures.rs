use nogine::{graphics::Graphics, window::{WindowCfg, WindowMode}, color::{Color4, Color}, math::Vector2, unwrap_res};

fn main() {
    // Create Window
    let mut window = unwrap_res!(WindowCfg::default().main(true).res((1280, 720)).title("Figures Example").mode(WindowMode::Windowed).init());

    while window.is_running() {
        window.pre_tick(None);
        
        Graphics::set_cam(Vector2::ZERO, Vector2(1.5 * window.aspect_ratio(), 1.5));

        Graphics::draw_rect(Vector2(-1.55, -0.5), Vector2::ONE, Color4::CYAN);
        Graphics::draw_circle(Vector2::ZERO, 0.5, Color4::YELLOW);
        Graphics::draw_polygon(Vector2(1.0, 0.0), 0.5, 0.0, 6, Color::PINK);
        
        window.post_tick();
    }
}