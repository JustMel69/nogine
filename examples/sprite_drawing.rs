use nogine::{window::WindowCfg, math::Vector2, graphics::{Graphics, texture::{Texture, TextureCfg, TextureFiltering, SpriteAtlas, SprRect}}, color::{Color4, Color}};

const ATLAS_TEX: &[u8] = include_bytes!("res/atlas.png");

fn main() {
    // Create Window
    let mut window = WindowCfg::default().main(true).res((1280, 720)).title("Sprite Drawing Example").mode(nogine::window::WindowMode::Windowed).init().expect("Couldn't open window");

    // Setup graphics
    Graphics::set_cam(Vector2::ZERO, 3.0);
    Graphics::set_pixels_per_unit(16.0);

    let tex = Texture::load(std::io::Cursor::new(ATLAS_TEX), TextureCfg { filtering: TextureFiltering::Closest, ..Default::default() });
    let atlas = SpriteAtlas::new(tex, (16, 16));

    while window.is_running() {
        // Refresh graphics
        Graphics::tick(window.aspect_ratio());
        window.clear_screen(Color4::BLACK);
        
        // Draw sprites
        Graphics::draw_sprite(Vector2(-2.0, -0.5), Vector2::ONE, 0.0, atlas.get(SprRect(0, 0, 1, 1)));
        Graphics::draw_sprite(Vector2(-1.0, -0.5), Vector2::ONE, 0.0, atlas.get(SprRect(1, 0, 1, 1)));
        Graphics::draw_sprite(Vector2( 0.0, -0.5), Vector2::ONE, 0.0, atlas.get(SprRect(0, 1, 1, 1)));
        Graphics::draw_sprite(Vector2( 1.0, -0.5), Vector2::ONE, 0.0, atlas.get(SprRect(1, 1, 1, 1)));
        
        // Handle window
        window.swap_buffers();
        window.handle_events();
    }
}