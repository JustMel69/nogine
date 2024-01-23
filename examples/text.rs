use nogine::{graphics::{Graphics, ui::text::{font::{BitmapFont, FontCfg}, HorTextAlignment, VerTextAlignment}, texture::{SpriteAtlas, Texture, TextureCfg, TextureFiltering, TextureWrapping}}, window::{WindowCfg, WindowMode}, color::{Color4, Color}, math::Vector2, unwrap_res};

const FONT_DATA: &[u8] = include_bytes!("res/text.png");

fn main() {
    // Create Window
    let mut window = unwrap_res!(WindowCfg::default().res((1280, 720)).title("Text Example").mode(WindowMode::Windowed).init());

    let font = BitmapFont::new(
        SpriteAtlas::new(
            unwrap_res!(Texture::load(
                std::io::Cursor::new(FONT_DATA),
                TextureCfg { filtering: TextureFiltering::Closest, wrapping: TextureWrapping::Clamp }
            )),
            (6, 8)
        ),
        "abcdefghijklmnopqrstuvwxyz",
        FontCfg{ monospace: true, char_spacing: 2.0 / 6.0, line_spacing: 4.0 / 8.0, ..Default::default()}
    );

    while window.is_running() {
        window.pre_tick(None);
        
        Graphics::set_cam(Vector2::ZERO, Vector2(1.5 * window.aspect_ratio(), 1.5));

        Graphics::set_pivot(Vector2::one(0.5));
        let (quad, _) = Graphics::text(Vector2::ZERO, Vector2(2.0, 1.6), 0.0, "one two three\nfour five six\nseven eight nine").font_size(0.1).font(&font).ver_align(VerTextAlignment::Expand).hor_align(HorTextAlignment::Expand).draw();
        Graphics::set_pivot(Vector2::ZERO);

        Graphics::draw_debug_quad(quad, Color4::LIME);
        
        window.post_tick();
    }
}