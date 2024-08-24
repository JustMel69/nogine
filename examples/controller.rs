use nogine::{color::{Color, Color4}, graphics::Graphics, input::Input, math::vec2, unwrap_res, window::WindowCfg};

fn main() {
    // Create Window
    let mut window = unwrap_res!(WindowCfg::default().title("Controller example").init());
    
    
    while window.is_running() {
        Graphics::set_cam(vec2::ZERO, vec2(window.aspect_ratio(), 1.0) * 5.0);

        window.pre_tick(None);

        if let Some(_ctrl) = Input::controller(0) {
            Graphics::draw_rect(vec2::one(-0.5), vec2::ONE, Color4::GREEN);
        } else {
            Graphics::draw_rect(vec2::one(-0.5), vec2::ONE, Color4::RED);
        }
        
        window.post_tick();
    }
}