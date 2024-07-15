use nogine::{color::{Color, Color4}, graphics::{Graphics, ui::text::{font::{BitmapFont, FontCfg}, HorTextAlignment, VerTextAlignment}, texture::{SpriteAtlas, Texture, TextureCfg, TextureFiltering, TextureWrapping}}, math::vec2, unwrap_res, window::{WindowCfg, WindowMode}};

const FONT_DATA: &[u8] = include_bytes!("res/text.png");
const NICE_FONT_DATA: &[u8] = include_bytes!("res/nice_text.png");

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

    let nice_font = BitmapFont::new(
        SpriteAtlas::new(
            unwrap_res!(Texture::load(
                std::io::Cursor::new(NICE_FONT_DATA),
                TextureCfg { filtering: TextureFiltering::Closest, wrapping: TextureWrapping::Clamp }
            )), 
            (10, 9)
        ),
        "0123456789.,:;'()[]{}<>?!¿¡_*+-=/#%@~ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyzÁÉÍÓÚÜáéíóúüÑñ",
        FontCfg { monospace: false, char_spacing: 1.0 / 9.0, line_spacing: 1.0 / 9.0, word_spacing: 5.0 / 9.0, ..Default::default() }
    );

    //window.set_target_framerate(Some(60.0));
    window.set_vsync(true);
    while window.is_running() {
        window.pre_tick(None);
        
        Graphics::set_cam(vec2::ZERO, vec2(1.5 * window.aspect_ratio(), 1.5));

        Graphics::set_pivot(vec2::one(0.5));

        Graphics::text(vec2::up(0.5), vec2(2.0, 0.15), 0.0, "monospace font").font_size(0.1).font(&font).ver_align(VerTextAlignment::Middle).hor_align(HorTextAlignment::Center).draw();

        let (quad, _) = Graphics::text(vec2::up(-0.5), vec2(3.0, 0.5), 0.0, "Very very long text that in no way in HELL fits in 12345 container bounds.\nBottom text.").font_size(0.1).font(&nice_font).ver_align(VerTextAlignment::Middle).hor_align(HorTextAlignment::Justified).word_wrapped().progress(Some(30)).draw();

        Graphics::set_pivot(vec2::ZERO);

        Graphics::draw_debug_quad(quad, Color4::LIME);
        
        window.post_tick();
    }
}