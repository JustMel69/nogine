use nogine::{color::{Color, Color4}, graphics::Graphics, input::{controller::{ControllerInput, ControllerSnapshot}, Input}, math::vec2, unwrap_res, window::WindowCfg};

fn main() {
    // Create Window
    let mut window = unwrap_res!(WindowCfg::default().title("Controller example").init());
    
    window.set_vsync(true);
    while window.is_running() {
        Graphics::set_cam(vec2::ZERO, vec2(window.aspect_ratio(), 1.0) * 5.0);

        window.pre_tick(None);

        if let Some(ctrl) = Input::controller(0) {
            draw_dpad(vec2(-6.5, 0.0), ctrl);
            draw_stick(vec2(-2.5, 0.0), ctrl.left_stick(), ctrl.button(ControllerInput::L3));
            draw_stick(vec2(2.5, 0.0), ctrl.right_stick(), ctrl.button(ControllerInput::R3));
            draw_abxy(vec2(6.5, 0.0), ctrl);
            draw_start_select(1.5, ctrl);
            draw_shoulder(2.0, ctrl);
        }
        
        window.post_tick();
        window.set_title(format!("Controller example ({} fps)", 1.0 / window.ts()).as_str());
    }
}

fn draw_stick(pos: vec2, stick: vec2, pressed: bool) {
    Graphics::draw_circle(pos, 1.0, Color4::DARK_GRAY);

    if pressed {
        Graphics::draw_circle(pos + stick, 0.15, Color4::GREEN);
    } else {
        Graphics::draw_circle(pos + stick, 0.05, Color4::RED);
    }
}

fn draw_dpad(pos: vec2, ctrl: ControllerSnapshot) {
    Graphics::set_pivot(vec2(1.0, 0.5));
    Graphics::draw_rect(pos + vec2(-0.25, 0.0), vec2(1.0, 0.5), if ctrl.button(ControllerInput::DPadLeft) { Color4::WHITE } else { Color4::DARK_GRAY });
    Graphics::set_pivot(vec2(0.0, 0.5));
    Graphics::draw_rect(pos + vec2(0.25, 0.0), vec2(1.0, 0.5), if ctrl.button(ControllerInput::DPadRight) { Color4::WHITE } else { Color4::DARK_GRAY });

    Graphics::set_pivot(vec2(0.5, 0.0));
    Graphics::draw_rect(pos + vec2(0.0, 0.25), vec2(0.5, 1.0), if ctrl.button(ControllerInput::DPadUp) { Color4::WHITE } else { Color4::DARK_GRAY });
    Graphics::set_pivot(vec2(0.5, 1.0));
    Graphics::draw_rect(pos + vec2(0.0, -0.25), vec2(0.5, 1.0), if ctrl.button(ControllerInput::DPadDown) { Color4::WHITE } else { Color4::DARK_GRAY });

    Graphics::set_pivot(vec2::ZERO);
}

fn draw_abxy(pos: vec2, ctrl: ControllerSnapshot) {
    Graphics::draw_circle(pos + vec2(1.0, 0.0), 0.5, if ctrl.button(ControllerInput::East) { Color4::WHITE } else { Color4::DARK_GRAY });
    Graphics::draw_circle(pos - vec2(1.0, 0.0), 0.5, if ctrl.button(ControllerInput::West) { Color4::WHITE } else { Color4::DARK_GRAY });
    Graphics::draw_circle(pos + vec2(0.0, 1.0), 0.5, if ctrl.button(ControllerInput::North) { Color4::WHITE } else { Color4::DARK_GRAY });
    Graphics::draw_circle(pos - vec2(0.0, 1.0), 0.5, if ctrl.button(ControllerInput::South) { Color4::WHITE } else { Color4::DARK_GRAY });
}

fn draw_start_select(y: f32, ctrl: ControllerSnapshot) {
    Graphics::set_pivot(vec2::one(0.5));
    Graphics::draw_rect(vec2(-2.5, y), vec2(1.0, 0.25), if ctrl.button(ControllerInput::Select) { Color4::WHITE } else { Color4::DARK_GRAY });
    Graphics::draw_rect(vec2(2.5, y), vec2(1.0, 0.25), if ctrl.button(ControllerInput::Start) { Color4::WHITE } else { Color4::DARK_GRAY });
    Graphics::set_pivot(vec2::ZERO);
}

fn draw_shoulder(y: f32, ctrl: ControllerSnapshot) {
    Graphics::set_pivot(vec2::one(0.5));
    Graphics::draw_rect(vec2(-6.5, y), vec2(2.0, 0.75), if ctrl.button(ControllerInput::L) { Color4::WHITE } else { Color4::DARK_GRAY });
    Graphics::draw_rect(vec2(6.5, y), vec2(2.0, 0.75), if ctrl.button(ControllerInput::R) { Color4::WHITE } else { Color4::DARK_GRAY });

    Graphics::draw_rect(vec2(-6.5, y + 1.0), vec2(2.0, 0.75), if ctrl.button(ControllerInput::L2) { Color4::WHITE } else { Color4::DARK_GRAY });
    Graphics::draw_rect(vec2(6.5, y + 1.0), vec2(2.0, 0.75), if ctrl.button(ControllerInput::R2) { Color4::WHITE } else { Color4::DARK_GRAY });
    Graphics::set_pivot(vec2::ZERO);
}