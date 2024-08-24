use std::{ffi::CStr, path::PathBuf, sync::RwLock};

use controller::ControllerSnapshot;
use controller_mapping::ControllerMappings;
use glfw::ffi::{glfwGetJoystickGUID, glfwJoystickPresent};

use crate::{assert_expr, math::vec2};

use super::gl_call;

pub mod controller;
pub mod controller_mapping;

#[derive(Clone, Copy, Debug)]
pub enum InputKind {
    Press, Hold, Release
}

impl Into<InputKind> for glfw::Action {
    fn into(self) -> InputKind {
        match self {
            glfw::Action::Release => InputKind::Release,
            glfw::Action::Press => InputKind::Press,
            glfw::Action::Repeat => InputKind::Hold,
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum MouseInput {
    Left = glfw::MouseButton::Button1 as u8,
    Right = glfw::MouseButton::Button2 as u8,
    Middle = glfw::MouseButton::Button3 as u8,
    Button4 = glfw::MouseButton::Button4 as u8,
    Button5 = glfw::MouseButton::Button5 as u8,
    Button6 = glfw::MouseButton::Button6 as u8,
    Button7 = glfw::MouseButton::Button7 as u8,
    Button8 = glfw::MouseButton::Button8 as u8,
}

impl Into<MouseInput> for glfw::MouseButton {
    fn into(self) -> MouseInput {
        return unsafe { std::mem::transmute(self as i32 as u8) }
    }
}


#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum KeyInput {
    Space = glfw::Key::Space as u32,
    Apostrophe = glfw::Key::Apostrophe as u32,
    Comma = glfw::Key::Comma as u32,
    Minus = glfw::Key::Minus as u32,
    Period = glfw::Key::Period as u32,
    Slash = glfw::Key::Slash as u32,
    Num0 = glfw::Key::Num0 as u32,
    Num1 = glfw::Key::Num1 as u32,
    Num2 = glfw::Key::Num2 as u32,
    Num3 = glfw::Key::Num3 as u32,
    Num4 = glfw::Key::Num4 as u32,
    Num5 = glfw::Key::Num5 as u32,
    Num6 = glfw::Key::Num6 as u32,
    Num7 = glfw::Key::Num7 as u32,
    Num8 = glfw::Key::Num8 as u32,
    Num9 = glfw::Key::Num9 as u32,
    Semicolon = glfw::Key::Semicolon as u32,
    Equal = glfw::Key::Equal as u32,
    A = glfw::Key::A as u32,
    B = glfw::Key::B as u32,
    C = glfw::Key::C as u32,
    D = glfw::Key::D as u32,
    E = glfw::Key::E as u32,
    F = glfw::Key::F as u32,
    G = glfw::Key::G as u32,
    H = glfw::Key::H as u32,
    I = glfw::Key::I as u32,
    J = glfw::Key::J as u32,
    K = glfw::Key::K as u32,
    L = glfw::Key::L as u32,
    M = glfw::Key::M as u32,
    N = glfw::Key::N as u32,
    O = glfw::Key::O as u32,
    P = glfw::Key::P as u32,
    Q = glfw::Key::Q as u32,
    R = glfw::Key::R as u32,
    S = glfw::Key::S as u32,
    T = glfw::Key::T as u32,
    U = glfw::Key::U as u32,
    V = glfw::Key::V as u32,
    W = glfw::Key::W as u32,
    X = glfw::Key::X as u32,
    Y = glfw::Key::Y as u32,
    Z = glfw::Key::Z as u32,
    LeftBracket = glfw::Key::LeftBracket as u32,
    Backslash = glfw::Key::Backslash as u32,
    RightBracket = glfw::Key::RightBracket as u32,
    GraveAccent = glfw::Key::GraveAccent as u32,
    World1 = glfw::Key::World1 as u32,
    World2 = glfw::Key::World2 as u32,

    Escape = glfw::Key::Escape as u32,
    Enter = glfw::Key::Enter as u32,
    Tab = glfw::Key::Tab as u32,
    Backspace = glfw::Key::Backspace as u32,
    Insert = glfw::Key::Insert as u32,
    Delete = glfw::Key::Delete as u32,
    Right = glfw::Key::Right as u32,
    Left = glfw::Key::Left as u32,
    Down = glfw::Key::Down as u32,
    Up = glfw::Key::Up as u32,
    PageUp = glfw::Key::PageUp as u32,
    PageDown = glfw::Key::PageDown as u32,
    Home = glfw::Key::Home as u32,
    End = glfw::Key::End as u32,
    CapsLock = glfw::Key::CapsLock as u32,
    ScrollLock = glfw::Key::ScrollLock as u32,
    NumLock = glfw::Key::NumLock as u32,
    PrintScreen = glfw::Key::PrintScreen as u32,
    Pause = glfw::Key::Pause as u32,
    F1 = glfw::Key::F1 as u32,
    F2 = glfw::Key::F2 as u32,
    F3 = glfw::Key::F3 as u32,
    F4 = glfw::Key::F4 as u32,
    F5 = glfw::Key::F5 as u32,
    F6 = glfw::Key::F6 as u32,
    F7 = glfw::Key::F7 as u32,
    F8 = glfw::Key::F8 as u32,
    F9 = glfw::Key::F9 as u32,
    F10 = glfw::Key::F10 as u32,
    F11 = glfw::Key::F11 as u32,
    F12 = glfw::Key::F12 as u32,
    F13 = glfw::Key::F13 as u32,
    F14 = glfw::Key::F14 as u32,
    F15 = glfw::Key::F15 as u32,
    F16 = glfw::Key::F16 as u32,
    F17 = glfw::Key::F17 as u32,
    F18 = glfw::Key::F18 as u32,
    F19 = glfw::Key::F19 as u32,
    F20 = glfw::Key::F20 as u32,
    F21 = glfw::Key::F21 as u32,
    F22 = glfw::Key::F22 as u32,
    F23 = glfw::Key::F23 as u32,
    F24 = glfw::Key::F24 as u32,
    F25 = glfw::Key::F25 as u32,
    Kp0 = glfw::Key::Kp0 as u32,
    Kp1 = glfw::Key::Kp1 as u32,
    Kp2 = glfw::Key::Kp2 as u32,
    Kp3 = glfw::Key::Kp3 as u32,
    Kp4 = glfw::Key::Kp4 as u32,
    Kp5 = glfw::Key::Kp5 as u32,
    Kp6 = glfw::Key::Kp6 as u32,
    Kp7 = glfw::Key::Kp7 as u32,
    Kp8 = glfw::Key::Kp8 as u32,
    Kp9 = glfw::Key::Kp9 as u32,
    KpDecimal = glfw::Key::KpDecimal as u32,
    KpDivide = glfw::Key::KpDivide as u32,
    KpMultiply = glfw::Key::KpMultiply as u32,
    KpSubtract = glfw::Key::KpSubtract as u32,
    KpAdd = glfw::Key::KpAdd as u32,
    KpEnter = glfw::Key::KpEnter as u32,
    KpEqual = glfw::Key::KpEqual as u32,
    LeftShift = glfw::Key::LeftShift as u32,
    LeftControl = glfw::Key::LeftControl as u32,
    LeftAlt = glfw::Key::LeftAlt as u32,
    LeftSuper = glfw::Key::LeftSuper as u32,
    RightShift = glfw::Key::RightShift as u32,
    RightControl = glfw::Key::RightControl as u32,
    RightAlt = glfw::Key::RightAlt as u32,
    RightSuper = glfw::Key::RightSuper as u32,
    Menu = glfw::Key::Menu as u32,
    Unknown = glfw::Key::Unknown as u32,
}

impl Into<KeyInput> for glfw::Key {
    fn into(self) -> KeyInput {
        return unsafe { std::mem::transmute(self) };
    }
}


static INPUT: RwLock<Input> = RwLock::new(Input::new());

pub struct Input {
    window_in: Vec<WindowInput>,

    keyboard_flags: [u64; 7],
    written_in: String,

    mouse_flags: u16,
    mouse_pos: vec2,
    scroll_in: vec2,
    
    controller: Option<(ControllerMappings, ControllerSnapshot)>,
}

impl Input {
    const fn new() -> Self {
        Self { keyboard_flags: [0; 7], window_in: vec![], written_in: String::new(), scroll_in: vec2::ZERO, mouse_flags: 0, mouse_pos: vec2::ZERO, controller: None }
    }

    pub(crate) fn flush() {
        let mut writer = INPUT.write().unwrap();
        Self::flush_keyboard(&mut writer);
        Self::flush_mouse(&mut writer);
        Self::flush_controllers(&mut writer);

        writer.window_in.clear();
        writer.written_in.clear();
        writer.scroll_in = vec2::ZERO;
    }



    // |>-<·>-<|      Keyboard Input      |>-<·>-<| //

    /// Checks if the key is being pressed.
    pub fn key(key: KeyInput) -> bool {
        let x = Self::key_state(key);
        return x == 0b01 || x == 0b11;
    }

    /// Returns an axis derived from the inputs of two keys.
    pub fn axis(neg: KeyInput, pos: KeyInput) -> i32 {
        let n = if Self::key(neg) { -1 } else { 0 };
        let p = if Self::key(pos) { 1 } else { 0 };
        return n + p;
    }

    /// Checks if the key has started to be pressed this frame.
    pub fn key_pressed(key: KeyInput) -> bool {
        let x = Self::key_state(key);
        return x == 0b01;
    }

    /// Checks if the key has been released this frame.
    pub fn key_released(key: KeyInput) -> bool {
        let x = Self::key_state(key);
        return x == 0b10;
    }

    /// Checks if the key is being held, altough for more than the current frame.
    pub fn key_hold(key: KeyInput) -> bool {
        let x = Self::key_state(key);
        return x == 0b11;
    }

    fn key_state(key: KeyInput) -> u8 {
        let reader = INPUT.read().unwrap();
        if matches!(key, KeyInput::Unknown) {
            return 0;
        }

        let (i, bit) = Self::key_to_pos(key);
        let flags = &reader.keyboard_flags[i];
        return ((*flags >> (bit * 2)) & 0b11) as u8;
    }

    fn set_key_state(writer: &mut std::sync::RwLockWriteGuard<'_, Input>, key: KeyInput, state: InputKind) {
        if matches!(key, KeyInput::Unknown) {
            return;
        }

        let (i, bit) = Self::key_to_pos(key);
        let flags = &mut writer.keyboard_flags[i];
        *flags &= !(0b11 << (bit * 2));

        let state: u64 = match state {
            InputKind::Press => 0b01,
            InputKind::Hold => 0b11,
            InputKind::Release => 0b10,
        };

        *flags |= state << (bit * 2);
    }

    fn key_to_pos(key: KeyInput) -> (usize, usize) {
        let mut bit_pos = key as u32 - 32;
        if bit_pos > 130 {
            bit_pos -= 93;
        }

        return ((bit_pos / 32) as usize, (bit_pos % 32) as usize);
    }

    fn flush_keyboard(writer: &mut std::sync::RwLockWriteGuard<'_, Input>) {
        /* Flushing the keyboard converts pressed keys to hold keys, and released keys to off keys.
         * Each key matches to two bytes in the flags to represent it's state.
         *
         * AB: State
         * 00: Off
         * 01: Pressed
         * 11: Hold
         * 10: Released
         * 
         * By using this system, the state can be flushed only by copying the bit B to A.
         */
        
        const FLUSH_MASK: u64 = 0x5555_5555_5555_5555; // 0b_01010_0101_0101...
        for flags in &mut writer.keyboard_flags {
            let mut x = *flags & FLUSH_MASK;
            x |= x << 1; // Copy bit B to A
            *flags = x;
        }
    }



    // |>-<·>-<|      Mouse Input      |>-<·>-<| //

    /// Checks if the button is being pressed.
    pub fn mouse(button: MouseInput) -> bool {
        let x = Self::mouse_state(button);
        return x == 0b01 || x == 0b11;
    }
    
    /// Checks if the button has started to be pressed this frame.
    pub fn mouse_pressed(button: MouseInput) -> bool {
        let x = Self::mouse_state(button);
        return x == 0b01;
    }
    
    /// Checks if the button has been released this frame.
    pub fn mouse_released(button: MouseInput) -> bool {
        let x = Self::mouse_state(button);
        return x == 0b10;
    }
    
    /// Checks if the button is being held, altough for more than the current frame.
    pub fn mouse_hold(button: MouseInput) -> bool {
        let x = Self::mouse_state(button);
        return x == 0b11;
    }

    /// Returns the mouse position in screen space.
    pub fn mouse_pos() -> vec2 {
        let reader = INPUT.read().unwrap();
        
        return reader.mouse_pos;
    }

    fn mouse_state(button: MouseInput) -> u8 {
        let reader = INPUT.read().unwrap();

        let bit = button as u8;
        return ((reader.mouse_flags >> (bit * 2)) & 0b11) as u8;
    }

    fn flush_mouse(writer: &mut std::sync::RwLockWriteGuard<'_, Input>) {
        const FLUSH_MASK: u16 = 0x5555; // 0b_01010_0101_0101...
        let mut x = writer.mouse_flags & FLUSH_MASK;
        x |= x << 1; // Copy bit B to A
        writer.mouse_flags = x;
    }

    fn set_mouse_state(writer: &mut std::sync::RwLockWriteGuard<'_, Input>, mouse: MouseInput, state: InputKind) {
        let bit = mouse as u8;
        writer.mouse_flags &= !(0b11 << (bit * 2));

        let state: u16 = match state {
            InputKind::Press => 0b01,
            InputKind::Hold => 0b11,
            InputKind::Release => 0b10,
        };

        writer.mouse_flags |= state << (bit * 2);
    }



    // |>-<·>-<|      Controller Input      |>-<·>-<| //

    pub fn controller(id: usize) -> Option<ControllerSnapshot> {
        assert_expr!(id == 0, "Only one controller is currently supported at the same time!");

        let mut writer = INPUT.write().unwrap();
        if let Some(x) = &writer.controller {
            return Some(x.1);
        }

        if unsafe { glfwJoystickPresent(0) } > 0 {
            let guid = unsafe { CStr::from_ptr(glfwGetJoystickGUID(0)) }.to_str().unwrap();
            let mapping = ControllerMappings::parse(guid)?;
            let layout = mapping.layout();
            writer.controller = Some((mapping, ControllerSnapshot::new(layout)));
            return Some(writer.controller.as_ref().unwrap().1);
        }
        
        return None;
    }

    fn flush_controllers(writer: &mut std::sync::RwLockWriteGuard<'_, Input>) {
        if let Some(x) = &mut writer.controller {
            x.1.flush();
            if !x.1.update(&x.0) {
                writer.controller = None;
            }
        }
    }




    pub fn get_last_window_input() -> Option<WindowInput> {
        let reader = INPUT.read().unwrap();
        return reader.window_in.last().cloned();
    }

    /// Gets the current mouse scroll (some devices have two scroll axis, for regular vertical just use the second axis)
    pub fn get_scroll() -> vec2 {
        let reader = INPUT.read().unwrap();
        return reader.scroll_in;
    }

    pub(crate) fn push_input(input: glfw::WindowEvent) {
        let mut writer = INPUT.write().unwrap();
        
        match input {
            glfw::WindowEvent::Pos(x, y) => writer.window_in.push(WindowInput::WinMove(x as u32, y as u32)),
            glfw::WindowEvent::Size(w, h) => writer.window_in.push(WindowInput::WinResize(w as u32, h as u32)),
            glfw::WindowEvent::Close => writer.window_in.push(WindowInput::WinClose),
            glfw::WindowEvent::Refresh => writer.window_in.push(WindowInput::WinRefresh),
            glfw::WindowEvent::Focus(b) => writer.window_in.push(if b { WindowInput::WinFocus } else { WindowInput::WinUnfocus }),
            glfw::WindowEvent::Iconify(b) => writer.window_in.push(if b { WindowInput::WinMinimize } else { WindowInput::WinDeminimize }),
            glfw::WindowEvent::FramebufferSize(w, h) => {
                writer.window_in.push(WindowInput::FramebufferResize(w as u32, h as u32));
                gl_call!(gl::Viewport(0, 0, w, h));
            },
            glfw::WindowEvent::FileDrop(p) => writer.window_in.push(WindowInput::FileDrop(p)),
            glfw::WindowEvent::Maximize(b) => writer.window_in.push(if b { WindowInput::WinMaximize } else { WindowInput::WinDemaximize }),
            glfw::WindowEvent::ContentScale(x, y) => writer.window_in.push(WindowInput::ContentScale(x, y)),
            glfw::WindowEvent::CursorPos(x, y) => writer.mouse_pos = vec2(x as f32, y as f32),

            glfw::WindowEvent::MouseButton(k, a, _) => Self::set_mouse_state(&mut writer, k.into(), a.into()),
            glfw::WindowEvent::Scroll(x, y) => writer.scroll_in = vec2(x as f32, y as f32),
            glfw::WindowEvent::Key(k, _, a, _) => Self::set_key_state(&mut writer, k.into(), a.into()),
            glfw::WindowEvent::Char(c) => writer.written_in.push(c),
            _ => {}
        }
    }
}



#[derive(Clone, PartialEq, Debug)]
pub enum WindowInput {
    WinMove(u32, u32),
    WinResize(u32, u32),
    WinClose,
    WinRefresh,
    WinFocus,
    WinUnfocus,
    WinMinimize,
    WinDeminimize,
    WinMaximize,
    WinDemaximize,
    FramebufferResize(u32, u32),
    FileDrop(Vec<PathBuf>),
    ContentScale(f32, f32),
}