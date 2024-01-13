use std::sync::RwLock;

use crate::{assert_expr, math::{Vector2, quad::Quad, Rect}, graphics::CamData, color::{Color4, Color}};

use super::render_scope::RenderScope;

macro_rules! assert_ui_enabled {
    () => {
        { 
            assert_expr!(UI_SINGLETON.read().unwrap().enabled, "UI must be enabled!");
        }
    };
}

pub(super) static UI_SINGLETON: RwLock<UI> = RwLock::new(UI::new());

pub struct UI {
    pub(super) enabled: bool,
    pub(super) scope: RenderScope,
    tint: Color4
}

impl UI {
    const fn new() -> Self {
        Self {
            enabled: false,
            scope: RenderScope::new(),
            tint: Color4::WHITE,
        }
    }

    pub fn enable() {
        UI_SINGLETON.write().unwrap().enabled = true;
    }

    pub fn is_enabled() -> bool {
        return UI_SINGLETON.read().unwrap().enabled;
    }

    pub fn draw_rect(origin: Origin, pos: Vector2, size: Vector2, color: Color4) -> Rect {
        assert_ui_enabled!();
        
        let mut writer = UI_SINGLETON.write().unwrap();
        let color = color * writer.tint;

        writer.scope.pivot = origin.get_pivot();
        let scope_pos = writer.process_pos(origin, pos);

        let quad = writer.scope.draw_rect(scope_pos, size, 0.0, [color; 4]);
        return writer.quad_to_rect(quad);
    }

    pub fn draw_debug_rect(rect: Rect, color: Color4) {
        assert_ui_enabled!();

        let (mut ld, mut lu, mut ru, mut rd) = (rect.ld(), rect.lu(), rect.ru(), rect.rd());
        ld.1 = -ld.1;
        rd.1 = -rd.1;
        ru.1 = -ru.1;
        lu.1 = -lu.1;
        
        let mut writer = UI_SINGLETON.write().unwrap();
        writer.scope.draw_line(ld, lu, [color; 2]);
        writer.scope.draw_line(lu, ru, [color; 2]);
        writer.scope.draw_line(ru, rd, [color; 2]);
        writer.scope.draw_line(rd, ld, [color; 2]);
    }

    pub fn set_tint(tint: Color4) {
        assert_ui_enabled!();
        UI_SINGLETON.write().unwrap().tint = tint;
    }

    pub fn get_tint() -> Color4 {
        assert_ui_enabled!();
        return UI_SINGLETON.read().unwrap().tint;
    }

    pub fn set_resolution(res: (u32, u32)) {
        assert_ui_enabled!();
        
        let half_size = Vector2(res.0 as f32, res.1 as f32) * 0.5;
        UI_SINGLETON.write().unwrap().scope.set_camera(CamData { pos: Vector2(half_size.0, -half_size.1 * 2.0), half_size });
    }

    pub fn get_resolution() -> (u32, u32) {
        assert_ui_enabled!();

        let size = UI_SINGLETON.read().unwrap().scope.cam_data.half_size * 2.0;
        return (size.0 as u32, size.1 as u32);
    }

    pub(crate) fn finalize_batch() {
        UI_SINGLETON.write().unwrap().scope.finalize_batch();
    }

    /// Converts from UI-space to scope-space
    fn process_pos(&self, origin: Origin, pos: Vector2) -> Vector2 {
        let origin_pivot = origin.get_pivot();
        let pivot = Vector2(origin_pivot.0, origin_pivot.1 - 1.5); // It just works

        return Vector2(pos.0, -pos.1) + pivot.scale(self.scope.cam_data.half_size * 2.0);
    }

    fn quad_to_rect(&self, quad: Quad) -> Rect {
        let mut pos = quad.ld;
        let size = quad.ru - quad.ld;
        pos.1 = -size.1 - pos.1;

        return Rect(pos.0, pos.1, size.0, size.1);
    }
}


#[derive(Debug, Clone, Copy)]
pub enum Origin {
    TopLeft, Top, TopRight,
    Left, Center, Right,
    BottomLeft, Bottom, BottomRight,
}

impl Origin {
    fn get_pivot(&self) -> Vector2 {
        match self {
            Origin::TopLeft => Vector2(0.0, 1.0),
            Origin::Top => Vector2(0.5, 1.0),
            Origin::TopRight => Vector2(1.0, 1.0),
            Origin::Left => Vector2(0.0, 0.5),
            Origin::Center => Vector2(0.5, 0.5),
            Origin::Right => Vector2(1.0, 0.5),
            Origin::BottomLeft => Vector2(0.0, 0.0),
            Origin::Bottom => Vector2(0.5, 0.0),
            Origin::BottomRight => Vector2(1.0, 0.0),
        }
    }
}