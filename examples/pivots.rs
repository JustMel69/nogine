use nogine::{window::WindowCfg, unwrap_res, graphics::Graphics, math::Vector2, color::{Color4, Color}};

fn main() {
    let mut window = unwrap_res!(WindowCfg::default().title("Pivots Example").init());

    let mut time = 0.0;
    while window.is_running() {
        window.pre_tick(None);

        Graphics::set_cam(Vector2::ZERO, Vector2(2.0 * window.aspect_ratio(), 2.0));

        time += window.ts();

        Graphics::set_pivot(Vector2::one(0.5));
        Graphics::draw_rect_full(Vector2::ZERO, Vector2::ONE, time, [Color4::RED; 4]);
        Graphics::set_pivot(Vector2::ZERO);

        window.post_tick();
    }
}