use nogine::{graphics::{Graphics, render_scope::RenderScope, texture::{TextureFiltering, Texture, TextureFormat, TextureCfg}}, window::{WindowCfg, WindowMode}, color::{Color4, Color}, math::vec2, unwrap_res};

fn main() {
    // Create Window
    let mut window = unwrap_res!(WindowCfg::default().res((1280, 720)).title("Figures Example").mode(WindowMode::Windowed).init());

    let mut scope_tex = Texture::empty(TextureFormat::RGBA, (256, 256), TextureCfg { filtering: TextureFiltering::Closest, ..Default::default() });
    let mut scope = RenderScope::new();
    Graphics::with_scope(&mut scope, || {
        Graphics::set_clear_col(Color4(0.5, 0.0, 0.0, 1.0));
        Graphics::set_cam(vec2::ZERO, vec2::one(1.5));
    });

    Graphics::set_pixels_per_unit(256.0);

    let mut time = 0.0f32;
    while window.is_running() {
        window.pre_tick(None);
        
        Graphics::set_cam(vec2::ZERO, vec2(1.5 * window.aspect_ratio(), 1.5));

        Graphics::with_scope(&mut scope, || {
            Graphics::draw_rect(vec2(-1.55, -0.5), vec2::ONE, Color4::CYAN);
            Graphics::draw_circle(vec2(0.0, time.sin()), 0.5, Color4::YELLOW);
            Graphics::draw_polygon(vec2(1.0, 0.0), 0.5, 0.0, 6, Color::PINK);
        });
        scope.rerender(&mut scope_tex, None);

        Graphics::set_pivot(vec2::one(0.5));
        Graphics::draw_texture(vec2::ZERO, vec2(time.cos(), 1.0), 0.0, &scope_tex);
        Graphics::set_pivot(vec2::ZERO);
        time += window.ts();
        
        window.post_tick();
    }
}