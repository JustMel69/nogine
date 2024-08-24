use crate::math::vec2;

use super::controller_mapping::ControllerLayout;

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
}