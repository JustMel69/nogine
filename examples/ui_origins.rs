use nogine::{graphics::ui::{UI, Origin}, window::{WindowCfg, WindowMode}, color::{Color4, Color}, math::Vector2, unwrap_res};

fn main() {
    // Create Window
    let mut window = unwrap_res!(WindowCfg::default().res((1280, 720)).title("UI Origins Example").mode(WindowMode::Windowed).init());

    UI::enable();

    while window.is_running() {
        window.pre_tick(None);
        
        UI::set_resolution(window.get_size());

        UI::draw_rect(Origin::TopLeft, Vector2::ZERO, Vector2::one(128.0), Color4::RED);
        UI::draw_rect(Origin::Top, Vector2::ZERO, Vector2::one(128.0), Color4::ORANGE);
        UI::draw_rect(Origin::TopRight, Vector2::ZERO, Vector2::one(128.0), Color4::YELLOW);

        UI::draw_rect(Origin::Left, Vector2::ZERO, Vector2::one(128.0), Color4::LIME);
        UI::draw_rect(Origin::Center, Vector2::ZERO, Vector2::one(128.0), Color4::GREEN);
        UI::draw_rect(Origin::Right, Vector2::ZERO, Vector2::one(128.0), Color4::CYAN);

        UI::draw_rect(Origin::BottomLeft, Vector2::ZERO, Vector2::one(128.0), Color4::BLUE);
        UI::draw_rect(Origin::Bottom, Vector2::ZERO, Vector2::one(128.0), Color4::PURPLE);
        UI::draw_rect(Origin::BottomRight, Vector2::ZERO, Vector2::one(128.0), Color4::PINK);
        
        window.post_tick();
    }
}