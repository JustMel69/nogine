use std::time::Instant;

use nogine::{color::{Color4, Color}, graphics::{Graphics, ui::text::{font::{BitmapFont, FontCfg}, HorTextAlignment, VerTextAlignment}, texture::{SpriteAtlas, Texture, TextureCfg, TextureFiltering, TextureFormat, TextureWrapping}}, log_info, math::Vector2, unwrap_res, utils::timer::Timer, window::{WindowCfg, WindowMode}};

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
        "0123456789.,:;'()[]{}<>?!¿¡_*+-=/#%@~ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyzÁÉÍÓÚáéíóúÑñ",
        FontCfg { monospace: false, char_spacing: 1.0 / 9.0, line_spacing: 1.0 / 9.0, word_spacing: 5.0 / 9.0, ..Default::default() }
    );

    window.set_target_framerate(Some(60.0));
    while window.is_running() {
        window.pre_tick(None);
        
        Graphics::set_cam(Vector2::ZERO, Vector2(1.5 * window.aspect_ratio(), 1.5));

        Graphics::set_pivot(Vector2::one(0.5));

        Graphics::text(Vector2::up(0.5), Vector2(2.0, 0.15), 0.0, "monospace font").font_size(0.1).font(&font).ver_align(VerTextAlignment::Middle).hor_align(HorTextAlignment::Center).draw();

        let timer = Timer::start("Text");
        Graphics::text(Vector2::up(-0.5), Vector2(4.0, 0.5), 0.0, "The Quick Brown Fox Jumps Over The Lazy Dog.\nthe quick brown fox jumps over the lazy dog?\nTHE QUICK BROWN FOX JUMPS OVER THE LAZY DOG!\nMe cago en la puta madre que te parió hijo de puta.").font_size(0.1).font(&nice_font).ver_align(VerTextAlignment::Middle).hor_align(HorTextAlignment::Center).draw();
        timer.end();

        Graphics::set_pivot(Vector2::ZERO);
        
        window.post_tick();
    }
}