use nogine::{window::WindowCfg, math::vec2, graphics::{Graphics, texture::Texture}, unwrap_res};

const TIMMY_TEX: &[u8] = include_bytes!("res/timmy.png");

fn main() {
    // Create Window
    let mut window = unwrap_res!(WindowCfg::default().res((1280, 720)).title("Texture Drawing Example").mode(nogine::window::WindowMode::Windowed).init());

    // Setup graphics
    Graphics::set_pixels_per_unit(200.0);
    
    let tex = unwrap_res!(Texture::load(std::io::Cursor::new(TIMMY_TEX), Default::default()));
    
    while window.is_running() {
        window.pre_tick(None);
        
        Graphics::set_cam(vec2::ZERO, vec2(1.5 * window.aspect_ratio(), 1.5));
        
        // Draw Texture
        Graphics::draw_texture(vec2(-0.5, -0.5), vec2::ONE, 0.0, &tex);
        
        window.post_tick();
    }
}