use std::{sync::RwLock, collections::HashMap};

use crate::{assert_expr, math::{Vector2, quad::Quad, Rect}, graphics::{CamData, Mode}, color::{Color4, Color}, log_info, input::{MouseInput, Input}};

use self::{internal::ActiveData, text::{Text, SourcedFromUI}};

use super::{render_scope::RenderScope, texture::{Texture, Sprite}};

macro_rules! assert_ui_enabled {
    () => {
        { 
            assert_expr!(UI_SINGLETON.read().unwrap().enabled, "UI must be enabled!");
        }
    };
}

pub mod text;

pub(super) static UI_SINGLETON: RwLock<UI> = RwLock::new(UI::new());

pub struct UI {
    pub(super) enabled: bool,
    pub(super) scope: RenderScope,
    tint: Color4,

    interactables: Vec<(String, Rect)>,
    inputs: Option<HashMap<String, Interaction>>,
    active_data: Option<internal::ActiveData>,

    prev_mouse: Vector2,
}

impl UI {
    const fn new() -> Self {
        Self {
            enabled: false,
            scope: RenderScope::new(),
            tint: Color4::WHITE,

            interactables: Vec::new(),
            inputs: None,
            active_data: None,

            prev_mouse: Vector2::ZERO,
        }
    }

    /// Enables the UI.<br>
    /// Calling this function at the start of the program is required for UI to work.
    pub fn enable() {
        let mut writer = UI_SINGLETON.write().unwrap();
        writer.enabled = true;
        writer.inputs = Some(HashMap::new());

        log_info!("UI initialized.")
    }

    /// Returns if the UI is enabled.
    pub fn is_enabled() -> bool {
        return UI_SINGLETON.read().unwrap().enabled;
    }

    /// Handles UI input.<br>
    /// This function should be called shortly before `post_tick` and after all `interactable`s have been created.
    pub fn handle_input(mouse_transform: impl Fn(Vector2) -> Option<Vector2>) {
        assert_ui_enabled!();
        let mut writer = UI_SINGLETON.write().unwrap();
        
        // --------------------------------- //
        //   ⚠️ ⚠️ ⚠️  UGLY CODE AHEAD  ⚠️ ⚠️ ⚠️   //
        //   [ Refactor is highly needed ]   //
        // --------------------------------- //

        let mut map = HashMap::new();
        let mut active_data = writer.active_data.clone();
        if let Some(mouse_pos) = mouse_transform(Input::mouse_pos()) {
            let pressed_flags = internal::get_clicks(Input::mouse_pressed);
            let mouse_scroll = Input::get_scroll();

            for inter in writer.interactables.iter().rev() {
                let hovering = inter.1.contains(mouse_pos);

                if let Some(active) = &writer.active_data {
                    if active.id != inter.0 {
                        continue;
                    }

                    if Input::mouse_released(active.input) {
                        if hovering {
                            internal::push_interaction(&mut map, &inter.0, Interaction::Click(active.input));
                        }
                        active_data = None;
                        break;
                    }

                    internal::push_interaction(&mut map, &inter.0, Interaction::DragOrHold {
                        input: active.input,
                        delta: Input::mouse_pos() - writer.prev_mouse
                    });
                    break;
                }

                if !hovering {
                    continue;
                }
            
                if mouse_scroll != Vector2::ZERO {
                    internal::push_interaction(&mut map, &inter.0, Interaction::Scroll(mouse_scroll));
                    break;
                }
            
                if let Some((click, _)) = internal::get_first_flag(pressed_flags) {
                    active_data = Some(ActiveData { id: inter.0.clone(), input: click });
                    break;
                }

                internal::push_interaction(&mut map, &inter.0, Interaction::Hover);
                break;
            }

            writer.prev_mouse = mouse_pos;
        }
        writer.inputs = Some(map);
        writer.interactables.clear();
        writer.active_data = active_data;
    }



    /// Draws a rectangle.
    pub fn draw_rect(origin: Origin, pos: Vector2, size: Vector2, color: Color4) -> Rect {
        assert_ui_enabled!();
        
        let mut writer = UI_SINGLETON.write().unwrap();
        let color = color * writer.tint;

        writer.scope.pivot = origin.get_pivot();
        let scope_pos = writer.process_pos(origin, pos);

        let quad = writer.scope.draw_rect(scope_pos, size, 0.0, [color; 4]);
        return writer.quad_to_rect(quad);
    }

    /// Draws a texture.
    pub fn draw_texture(origin: Origin, pos: Vector2, scale: Vector2, texture: &Texture) -> Rect {
        Self::draw_texture_ext(origin, pos, scale, Rect::IDENT, texture)
    }

    /// Draw a sprite.
    pub fn draw_sprite(origin: Origin, pos: Vector2, scale: Vector2, sprite: Sprite<'_>) -> Rect {
        let rect = sprite.rect();
        let tex = sprite.tex();

        return Self::draw_texture_ext(origin, pos, scale, rect, tex);
    }

    /// Draws a texture with extended control.
    pub fn draw_texture_ext(origin: Origin, pos: Vector2, scale: Vector2, uv_rect: Rect, texture: &Texture) -> Rect {
        assert_ui_enabled!();

        let mut writer = UI_SINGLETON.write().unwrap();
        let color = writer.tint;

        writer.scope.pivot = origin.get_pivot();
        let scope_pos = writer.process_pos(origin, pos);

        let quad = writer.scope.draw_texture(scope_pos, scale, 0.0, uv_rect, [color; 4], texture);
        return writer.quad_to_rect(quad);
    }

    /// Draws a panel.
    pub fn draw_panel(origin: Origin, pos: Vector2, size: Vector2, sprite: Sprite<'_>, scaling: f32) -> Rect {
        assert_ui_enabled!();
        let corner_dims = Vector2::from(sprite.tex().dims()).scale(sprite.rect().size()) / 3.0 * scaling;
        let size = Vector2(size.0.max(corner_dims.0 * 2.0), size.1.max(corner_dims.1 * 2.0));
        
        let mut writer = UI_SINGLETON.write().unwrap();

        writer.scope.pivot = origin.get_pivot();
        let scope_pos = writer.process_pos(origin, pos);

        let quad = writer.scope.rect_positions(scope_pos, size, 0.0, false);
        
        #[repr(C)]
        #[derive(Debug)]
        struct Vert(Vector2, Color4, Vector2);

        let tint = writer.tint;
        let rect = sprite.rect();

        let xpos = [ quad.lu.0, quad.lu.0 + corner_dims.0, quad.ru.0 - corner_dims.0, quad.ru.0 ];
        let ypos = [ quad.ld.1, quad.ld.1 - corner_dims.1, quad.lu.1 + corner_dims.1, quad.lu.1 ];
        let xuvs = [ rect.0, rect.0 + rect.2 * 1.0 / 3.0, rect.0 + rect.2 * 2.0 / 3.0, rect.0 + rect.2 ];
        let yuvs = [ rect.1, rect.1 + rect.3 * 1.0 / 3.0, rect.1 + rect.3 * 2.0 / 3.0, rect.1 + rect.3 ];

        let verts = [
            Vert(Vector2(xpos[0], ypos[3]), tint, Vector2(xuvs[0], yuvs[0])),
            Vert(Vector2(xpos[1], ypos[3]), tint, Vector2(xuvs[1], yuvs[0])),
            Vert(Vector2(xpos[2], ypos[3]), tint, Vector2(xuvs[2], yuvs[0])),
            Vert(Vector2(xpos[3], ypos[3]), tint, Vector2(xuvs[3], yuvs[0])),

            Vert(Vector2(xpos[0], ypos[2]), tint, Vector2(xuvs[0], yuvs[1])),
            Vert(Vector2(xpos[1], ypos[2]), tint, Vector2(xuvs[1], yuvs[1])),
            Vert(Vector2(xpos[2], ypos[2]), tint, Vector2(xuvs[2], yuvs[1])),
            Vert(Vector2(xpos[3], ypos[2]), tint, Vector2(xuvs[3], yuvs[1])),

            Vert(Vector2(xpos[0], ypos[1]), tint, Vector2(xuvs[0], yuvs[2])),
            Vert(Vector2(xpos[1], ypos[1]), tint, Vector2(xuvs[1], yuvs[2])),
            Vert(Vector2(xpos[2], ypos[1]), tint, Vector2(xuvs[2], yuvs[2])),
            Vert(Vector2(xpos[3], ypos[1]), tint, Vector2(xuvs[3], yuvs[2])),

            Vert(Vector2(xpos[0], ypos[0]), tint, Vector2(xuvs[0], yuvs[3])),
            Vert(Vector2(xpos[1], ypos[0]), tint, Vector2(xuvs[1], yuvs[3])),
            Vert(Vector2(xpos[2], ypos[0]), tint, Vector2(xuvs[2], yuvs[3])),
            Vert(Vector2(xpos[3], ypos[0]), tint, Vector2(xuvs[3], yuvs[3])),
        ];
        let verts = internal::convert_vert_data(&verts);

        const TRIS: &[u32] = &[
            4, 0, 1, 1, 5, 4,
            5, 1, 2, 2, 6, 5,
            6, 2, 3, 3, 7, 6,

            8, 4, 5, 5, 9, 8,
            9, 5, 6, 6, 10, 9,
            10, 6, 7, 7, 11, 10,

            12, 8, 9, 9, 13, 12,
            13, 9, 10, 10, 14, 13,
            14, 10, 11, 11, 15, 14,
        ];

        unsafe { writer.scope.draw_manual(Mode::Textured, verts, TRIS, &[2, 4, 2], &[sprite.tex()]) };
        return writer.quad_to_rect(internal::fix_quad(quad));
    }

    /// Creates a new text.
    pub fn text(pos: Vector2, bounds_size: Vector2, text: &str) -> Text<'_, SourcedFromUI> {
        let tint = { UI_SINGLETON.read().unwrap().tint };
        return Text::<'_, SourcedFromUI>::new(pos, bounds_size, tint, text);
    }

    /// Draws a wireframe rect.
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

    /// Handles interaction with a rect.<br>
    /// - `id` should be tried to be kept unique, at least to the extent where no interactables have the same id at the same time.
    pub fn interactable(rect: Rect, id: impl Into<String>) -> Option<Interaction> {
        assert_ui_enabled!();
        
        let mut writer = UI_SINGLETON.write().unwrap();

        let id = id.into();
        let input = writer.inputs.as_ref().unwrap().get(&id).cloned();

        writer.interactables.push((id, rect));
        return input;
    }

    /// Sets the UI tint.
    pub fn set_tint(tint: Color4) {
        assert_ui_enabled!();
        UI_SINGLETON.write().unwrap().tint = tint;
    }

    /// Returns the UI tint.
    pub fn get_tint() -> Color4 {
        assert_ui_enabled!();
        return UI_SINGLETON.read().unwrap().tint;
    }

    /// Sets the UI resolution.
    pub fn set_resolution(res: (u32, u32)) {
        assert_ui_enabled!();
        
        let half_size = Vector2(res.0 as f32, res.1 as f32) * 0.5;
        UI_SINGLETON.write().unwrap().scope.set_camera(CamData { pos: Vector2(half_size.0, -half_size.1), half_size });
    }

    /// Returns the UI resolution.
    pub fn get_resolution() -> (u32, u32) {
        assert_ui_enabled!();

        let size = UI_SINGLETON.read().unwrap().scope.cam_data.half_size * 2.0;
        return (size.0 as u32, size.1 as u32);
    }

    pub(crate) fn finalize_batch() {
        UI_SINGLETON.write().unwrap().scope.finalize_batch();
    }

    pub(crate) fn using_singleton<T>(func: impl Fn(&mut UI) -> T) -> T {
        let mut writer = UI_SINGLETON.write().unwrap();
        return func(&mut writer);
    }

    /// Converts from UI-space to scope-space
    fn process_pos(&self, origin: Origin, pos: Vector2) -> Vector2 {
        let origin_pivot = origin.get_pivot();
        let pivot = Vector2(origin_pivot.0, origin_pivot.1 - 1.0); // It just works (before it didn't actually worked, not it actually does :D)

        return Vector2(pos.0, -pos.1) + pivot.scale(self.scope.cam_data.half_size * 2.0);
    }

    pub(crate) fn quad_to_rect(&self, quad: Quad) -> Rect {
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


#[derive(Debug, Clone)]
pub enum Interaction {
    Hover,
    Click(MouseInput), Scroll(Vector2),
    DragOrHold { input: MouseInput, delta: Vector2 }
}

impl Interaction {
    pub fn is_active(&self) -> bool {
        return matches!(self, Interaction::Hover | Interaction::DragOrHold { input: _, delta: _ })
    }
}



mod internal {
    use std::collections::HashMap;

    use crate::{math::{quad::Quad, Vector2}, input::MouseInput, log_warn};

    use super::Interaction;

    #[derive(Clone)]
    pub struct ActiveData {
        pub id: String,
        pub input: MouseInput,
    }

    pub fn convert_vert_data<T>(src: &[T]) -> &[f32] {
        let mul = std::mem::size_of::<T>() / std::mem::size_of::<f32>();
        return unsafe { std::slice::from_raw_parts(src.as_ptr() as *const f32, src.len() * mul) };
    }

    pub fn fix_quad(quad: Quad) -> Quad {
        return Quad {
            ld: Vector2(quad.ld.0, -quad.ld.1),
            rd: Vector2(quad.rd.0, -quad.rd.1),
            lu: Vector2(quad.lu.0, -quad.lu.1),
            ru: Vector2(quad.ru.0, -quad.ru.1),
        }
    }

    pub fn get_clicks(func: fn(MouseInput) -> bool) -> u8 {
        return 
            ((func(MouseInput::Left) as u8) << 0u8) |
            ((func(MouseInput::Right) as u8) << 1u8) |
            ((func(MouseInput::Middle) as u8) << 2u8) |
            ((func(MouseInput::Button4) as u8) << 3u8) |
            ((func(MouseInput::Button5) as u8) << 4u8) |
            ((func(MouseInput::Button6) as u8) << 5u8) |
            ((func(MouseInput::Button7) as u8) << 6u8) |
            ((func(MouseInput::Button8) as u8) << 7u8);
    }

    pub fn get_first_flag(flags: u8) -> Option<(MouseInput, u8)> {
        if (flags & 1) != 0 { return Some((MouseInput::Left, 1)) };
        if (flags & 2) != 0 { return Some((MouseInput::Right, 2)) };
        if (flags & 4) != 0 { return Some((MouseInput::Middle, 4)) };
        if (flags & 8) != 0 { return Some((MouseInput::Button4, 8)) };
        if (flags & 16) != 0 { return Some((MouseInput::Button5, 16)) };
        if (flags & 32) != 0 { return Some((MouseInput::Button6, 32)) };
        if (flags & 64) != 0 { return Some((MouseInput::Button7, 64)) };
        if (flags & 128) != 0 { return Some((MouseInput::Button8, 128)) };
        return None;
    }

    pub fn push_interaction(map: &mut HashMap<String, Interaction>, id: &String, inter: Interaction) {
        let opt = map.insert(id.clone(), inter);

        if opt.is_some() {
            log_warn!("Interactable \"{id}\" was found more than one time!");
        }
    }
}