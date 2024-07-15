use nogine::{graphics::Graphics, window::{WindowCfg, WindowMode}, color::{Color4, Color}, math::vec2, unwrap_res};

fn main() {
    // Create Window
    let mut window = unwrap_res!(WindowCfg::default().res((1280, 720)).title("Figures Example").mode(WindowMode::Windowed).init());

    while window.is_running() {
        window.pre_tick(None);
        
        Graphics::set_cam(vec2::ZERO, vec2(1.5 * window.aspect_ratio(), 1.5));

        Graphics::draw_rect(vec2(-1.55, -0.5), vec2::ONE, Color4::CYAN);
        Graphics::draw_circle(vec2::ZERO, 0.5, Color4::YELLOW);
        Graphics::draw_polygon(vec2(1.0, 0.0), 0.5, 0.0, 6, Color::PINK);
        
        window.post_tick();
    }
}