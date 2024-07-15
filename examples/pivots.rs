use nogine::{window::WindowCfg, unwrap_res, graphics::Graphics, math::vec2, color::{Color4, Color}};

fn main() {
    let mut window = unwrap_res!(WindowCfg::default().title("Pivots Example").init());

    let mut time = 0.0;
    while window.is_running() {
        window.pre_tick(None);

        Graphics::set_cam(vec2::ZERO, vec2(2.0 * window.aspect_ratio(), 2.0));

        time += window.ts();

        Graphics::set_pivot(vec2::one(0.5));
        Graphics::draw_rect_full(vec2::ZERO, vec2::ONE, time, [Color4::RED; 4]);
        Graphics::set_pivot(vec2::ZERO);

        window.post_tick();
    }
}