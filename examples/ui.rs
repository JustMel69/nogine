use nogine::{graphics::{ui::{UI, Origin, Interaction}, texture::{SpriteAtlas, Texture, TextureCfg, TextureFiltering, SprRect, Sprite}, Graphics}, window::{WindowCfg, WindowMode}, color::{Color4, Color}, math::Vector2, unwrap_res, utils::rng::RNG};

const HEARTS_DATA: &[u8] = include_bytes!("res/hearts.png");
const PANEL_DATA: &[u8] = include_bytes!("res/panel.png");

struct Assets {
    hearts: SpriteAtlas,
    panel: SpriteAtlas,
}

impl Assets {
    fn load() -> Self {
        let hearts = SpriteAtlas::new(
            unwrap_res!(Texture::load(
                std::io::Cursor::new(HEARTS_DATA),
                TextureCfg { filtering: TextureFiltering::Closest, ..Default::default() }
            )),            
            (12, 12)
        );
    
        let panel = SpriteAtlas::new(
            unwrap_res!(Texture::load(
                std::io::Cursor::new(PANEL_DATA),
                TextureCfg { filtering: TextureFiltering::Linear, ..Default::default() }
            )),            
            (32, 32)
        );

        return Self { hearts, panel };
    }
}

struct Resources {
    health: u32,
    hovering: bool,
}

fn main() {
    // Create Window
    let mut window = unwrap_res!(WindowCfg::default().res((1280, 720)).title("UI Example").mode(WindowMode::Windowed).init());

    UI::enable();

    let assets = Assets::load();
    let mut resources = Resources { health: 0, hovering: false };

    let mut time = 0.0;
    while window.is_running() {
        window.pre_tick(None);
        
        resources.health = (time * 5.0) as u32 % 21;

        draw_ui(window.get_size(), &mut resources, &assets);
        
        UI::handle_input(|x| Some(x));
        time += window.ts();
        window.post_tick();
    }
}

fn draw_ui(res: (u32, u32), resources: &mut Resources, assets: &Assets) {
    UI::set_resolution(res);

    draw_health_bar(resources.health, &assets.hearts);
    draw_button(&mut resources.hovering, assets.panel.get(SprRect(0, 0, 3, 3)));
}

fn draw_health_bar(health: u32, atlas: &SpriteAtlas) {
    UI::draw_rect(Origin::TopLeft, Vector2::ZERO, Vector2(30.0 * 12.0 + 3.0, 36.0), Color4::GRAY);
    for i in 0..10 {
        let id = if i * 2 >= health {
            0
        } else if i * 2 + 1 == health {
            1
        } else {
            2
        };
        UI::draw_sprite(Origin::TopLeft, Vector2(3.0 + i as f32 * 36.0, 0.0), Vector2::one(3.0), atlas.get(SprRect(id as u32, 0, 1, 1)));
    }
}

fn draw_button(hovering: &mut bool, sprite: Sprite<'_>) {
    if *hovering {
        UI::set_tint(Color4::GRAY);
    }
    let rect = UI::draw_panel(Origin::Bottom, Vector2::ZERO, Vector2(500.0, 100.0), sprite, 1.0);
    let interaction = UI::interactable(rect, "button");
    
    *hovering = interaction.as_ref().map(|x| x.is_active()).unwrap_or(false);
    if matches!(interaction, Some(Interaction::Click(_))) {
        let rng = RNG::global();
        Graphics::set_clear_col(Color4(rng.gen(), rng.gen(), rng.gen(), 1.0));
    }

    UI::set_tint(Color4::WHITE);
}