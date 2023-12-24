use nogine::{graphics::Graphics, window::{WindowCfg, WindowMode}, color::{Color4, Color}, math::Vector2, unwrap_res, input::{Input, KeyInput}};

fn main() {
    // Create Window
    let mut window = unwrap_res!(WindowCfg::default().res((1280, 720)).title("Lines Example").mode(WindowMode::Windowed).init());

    let mut cam_pos = Vector2::ZERO;
    while window.is_running() {
        window.pre_tick(None);
        
        Graphics::set_cam(cam_pos, Vector2(1.5 * window.aspect_ratio(), 1.5));

        Graphics::draw_rect(Vector2::one(-0.5), Vector2::ONE, Color4::BLUE);

        Graphics::draw_line(Vector2(-2.0, -1.0), Vector2(-1.0, 0.0), Color4::PINK);
        Graphics::draw_line_ext(Vector2(-1.0, 0.0), Vector2(1.0, 0.0), [Color4::PINK, Color4::YELLOW]);
        Graphics::draw_line(Vector2(1.0, 0.0), Vector2(2.0, 1.0), Color4::YELLOW);

        cam_pos.0 += Input::axis(KeyInput::A, KeyInput::D) as f32 * window.ts();
        cam_pos.1 += Input::axis(KeyInput::S, KeyInput::W) as f32 * window.ts();

        window.post_tick();
    }
}