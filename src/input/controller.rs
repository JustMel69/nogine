use glfw::ffi::{glfwGetJoystickAxes, glfwGetJoystickButtons, glfwGetJoystickHats};

use crate::math::vec2;

use super::controller_mapping::{ControllerLayout, ControllerMappings};

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum ControllerInput {
    A, B, X, Y,
    Cross, Square, Circle, Triangle,
    /// Northern button, matches to ``X`` in Nintendo, ``Y`` in Xbox and ``Triangle`` in Playstation.
    North,
    /// Southern button, matches to ``B`` in Nintendo, ``A`` in Xbox and ``Cross`` in Playstation.
    South,
    /// Eastern button, matches to ``A`` in Nintendo, ``B`` in Xbox and ``Circle`` in Playstation.
    East,
    /// Western button, matches to ``Y`` in Nintendo, ``X`` in Xbox and ``Square`` in Playstation.
    West,
    /// Also known as `L1`, `Left Bumper` or `Left Shoulder`.
    L,
    /// Also known as `Left Trigger`.
    L2,
    /// Also known as `Left Stick`.
    L3,
    /// Also known as `R1`, `Right Bumper` or `Right Shoulder`.
    R,
    /// Also known as `Right Trigger`.
    R2,
    /// Also known as `Right Stick`.
    R3,
    DPadDown, DPadUp, DPadRight, DPadLeft,
    /// Also known as `+`.
    Start,
    /// Also known as `-`.
    Select,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
enum InternalControllerInput {
    A, B, X, Y, L, L2, L3, R, R2, R3, DPadDown, DPadUp, DPadRight, DPadLeft, Start, Select
}

impl InternalControllerInput {
    const MAPS: [&'static [Self]; 3] = [
        &[Self::A, Self::B, Self::X, Self::Y, Self::B, Self::Y, Self::A, Self::X, Self::X, Self::B, Self::A, Self::Y, Self::L, Self::L2, Self::L3, Self::R, Self::R2, Self::R3, Self::DPadDown, Self::DPadUp, Self::DPadRight, Self::DPadLeft, Self::Start, Self::Select],
        &[Self::A, Self::B, Self::X, Self::Y, Self::A, Self::X, Self::B, Self::Y, Self::Y, Self::A, Self::B, Self::X, Self::L, Self::L2, Self::L3, Self::R, Self::R2, Self::R3, Self::DPadDown, Self::DPadUp, Self::DPadRight, Self::DPadLeft, Self::Start, Self::Select],
        &[Self::A, Self::B, Self::X, Self::Y, Self::A, Self::X, Self::B, Self::Y, Self::Y, Self::A, Self::B, Self::X, Self::L, Self::L2, Self::L3, Self::R, Self::R2, Self::R3, Self::DPadDown, Self::DPadUp, Self::DPadRight, Self::DPadLeft, Self::Start, Self::Select],
    ];
    
    fn new(input: ControllerInput, layout: ControllerLayout) -> Self {
        return Self::MAPS[layout as u8 as usize][input as u8 as usize];
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ControllerSnapshot {
    left_stick: vec2,
    right_stick: vec2,
    button_flags: u32,
    layout: ControllerLayout,
}

impl ControllerSnapshot {
    pub fn new(layout: ControllerLayout) -> Self {
        Self { left_stick: vec2::ZERO, right_stick: vec2::ZERO, button_flags: 0, layout }
    }

    pub fn left_stick(&self) -> vec2 {
        self.left_stick
    }
    
    pub fn right_stick(&self) -> vec2 {
        self.right_stick
    }
    
    pub fn layout(&self) -> ControllerLayout {
        self.layout
    }

    /// Checks if the button is being pressed.
    pub fn button(&self, button: ControllerInput) -> bool {
        let x = self.button_state(InternalControllerInput::new(button, self.layout));
        return x == 0b01 || x == 0b11;
    }

    /// Checks if the button has been released this frame.
    pub fn button_released(&self, button: ControllerInput) -> bool {
        let x = self.button_state(InternalControllerInput::new(button, self.layout));
        return x == 0b10;
    }
    
    /// Checks if the button has started to be pressed this frame.
    pub fn button_pressed(&self, button: ControllerInput) -> bool {
        let x = self.button_state(InternalControllerInput::new(button, self.layout));
        return x == 0b01;
    }

    /// Checks if the button is being held, altough for more than the current frame.
    pub fn button_hold(&self, button: ControllerInput) -> bool {
        let x = self.button_state(InternalControllerInput::new(button, self.layout));
        return x == 0b11;
    }

    /// Returns an axis derived from the inputs of two buttons.
    pub fn axis(&self, neg: ControllerInput, pos: ControllerInput) -> i32 {
        let n = if self.button(neg) { -1 } else { 0 };
        let p = if self.button(pos) { 1 } else { 0 };
        return n + p;
    }

    fn button_state(&self, button: InternalControllerInput) -> u8 {
        let bit = button as u32;
        return ((self.button_flags >> (bit * 2)) & 0b11) as u8;
    }

    pub(super) fn flush(&mut self) {
        const FLUSH_MASK: u32 = 0x5555_5555; // 0b_01010_0101_0101...
        let mut x = self.button_flags & FLUSH_MASK;
        x |= x << 1; // Copy bit B to A
        self.button_flags = x;
    }

    pub(super) fn update(&mut self, mapping: &ControllerMappings) -> bool {
        let mut axes_count = 0;
        let axes = unsafe { glfwGetJoystickAxes(0, &mut axes_count) };
        if axes.is_null() {
            return false;
        }

        {
            let left_stick = mapping.left_stick();
            self.left_stick = unsafe { vec2(
                if left_stick.0 != -1 && left_stick.0 < axes_count { axes.add(left_stick.0 as usize).read() } else { 0.0 },
                if left_stick.1 != -1 && left_stick.1 < axes_count { -axes.add(left_stick.1 as usize).read() } else { 0.0 },
            ) };
        }

        {
            let right_stick = mapping.right_stick();
            self.right_stick = unsafe { vec2(
                if right_stick.0 != -1 && right_stick.0 < axes_count { axes.add(right_stick.0 as usize).read() } else { 0.0 },
                if right_stick.1 != -1 && right_stick.1 < axes_count { -axes.add(right_stick.1 as usize).read() } else { 0.0 },
            ) };
        }

        let mut button_count = 0;
        let buttons = unsafe { glfwGetJoystickButtons(0, &mut button_count) };

        macro_rules! set_button_flag {
            ($map:expr, $input:expr) => {
                {
                    let x = $map;
                    if x != -1 && x < button_count { self.set_state_flag($input, unsafe { buttons.add(x as usize).read() } > 0) }
                }
            };
        }

        set_button_flag!(mapping.a(), InternalControllerInput::A);
        set_button_flag!(mapping.b(), InternalControllerInput::B);
        set_button_flag!(mapping.x(), InternalControllerInput::X);
        set_button_flag!(mapping.y(), InternalControllerInput::Y);

        set_button_flag!(mapping.l1(), InternalControllerInput::L);
        set_button_flag!(mapping.l2(), InternalControllerInput::L2);
        set_button_flag!(mapping.l3(), InternalControllerInput::L3);

        set_button_flag!(mapping.r1(), InternalControllerInput::R);
        set_button_flag!(mapping.r2(), InternalControllerInput::R2);
        set_button_flag!(mapping.r3(), InternalControllerInput::R3);

        set_button_flag!(mapping.start(), InternalControllerInput::Start);
        //set_button_flag!(mapping.select(), InternalControllerInput::Select);

        let mut hat_count = 0;
        let hats = unsafe { glfwGetJoystickHats(0, &mut hat_count) };
        
        if hat_count >= 1 {
            let hat_val = unsafe { hats.read() } as i32;
            if mapping.dpad().up() != -1 { self.set_state_flag(InternalControllerInput::DPadUp, (hat_val & mapping.dpad().up()) > 0) };
            if mapping.dpad().right() != -1 { self.set_state_flag(InternalControllerInput::DPadRight, (hat_val & mapping.dpad().right()) > 0) };
            if mapping.dpad().down() != -1 { self.set_state_flag(InternalControllerInput::DPadDown, (hat_val & mapping.dpad().down()) > 0) };
            if mapping.dpad().left() != -1 { self.set_state_flag(InternalControllerInput::DPadLeft, (hat_val & mapping.dpad().left()) > 0) };
        }

        return true;
    }

    fn set_state_flag(&mut self, button: InternalControllerInput, flag: bool) {
        let bit = button as u32;
        self.button_flags &= !(0b1 << (bit * 2));
        self.button_flags |= (flag as u32) << (bit * 2);
    }
}