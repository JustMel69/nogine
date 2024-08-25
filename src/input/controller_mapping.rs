use crate::math::ivec2;

use super::controller::{ControllerInput, ResolvedControllerInput};

const SDL_GAMECONTROLLER_DB: &'static str = include_str!("../../vendor/SDL_GameControllerDB/gamecontrollerdb.txt");

#[derive(Debug, Clone, Copy)]
pub enum ControllerLayout {
    Nintendo,
    Playstation,
    Xbox,
}

impl ControllerLayout {
    pub fn by_name(str: &str) -> Self {
        let str = str.to_lowercase();

        if str.contains("playstation") || str.contains("ps2") || str.contains("ps3") || str.contains("ps4") || str.contains("ps5") || str.contains("psx") || str.contains("dualshock") {
            return Self::Playstation;
        }

        if str.contains("gamecube") || str.contains("switch") || str.contains("nintendo") || str.contains("n64") || str.contains("wii u") || str.contains("wii") {
            return Self::Nintendo;
        }

        return Self::Xbox;
    }

    pub fn resolve_imput(&self, input: ControllerInput) -> ResolvedControllerInput {
        return ResolvedControllerInput::new(input, *self);
    }
}


pub struct DirMappings {
    left: i32,
    right: i32,
    up: i32,
    down: i32,
}

impl DirMappings {
    pub fn left(&self) -> i32 {
        self.left
    }
    
    pub fn right(&self) -> i32 {
        self.right
    }
    
    pub fn up(&self) -> i32 {
        self.up
    }
    
    pub fn down(&self) -> i32 {
        self.down
    }
}

impl Default for DirMappings {
    fn default() -> Self {
        Self { left: -1, right: -1, up: -1, down: -1 }
    }
}

pub struct ControllerMappings {
    layout: ControllerLayout,
    name: String,

    a: i32,
    b: i32,
    x: i32,
    y: i32,

    dpad: DirMappings,
    left_stick: ivec2,
    right_stick: ivec2,

    l1: i32,
    l2: i32,
    l3: i32,

    r1: i32,
    r2: i32,
    r3: i32,

    start: i32,
    select: i32,
}

#[cfg(target_os = "windows")] const PLATFORM: &'static str = "platform:Windows,";
#[cfg(target_os = "macos")] const PLATFORM: &'static str = "platform:Mac OS X,";
#[cfg(target_os = "linux")] const PLATFORM: &'static str = "platform:Linux,";
#[cfg(target_os = "android")] const PLATFORM: &'static str = "platform:Android,";
#[cfg(target_os = "ios")] const PLATFORM: &'static str = "platform:iOS,";

impl ControllerMappings {
    pub fn parse(guid: &str) -> Option<Self> {
        let mut res = Self::default();
        
        for line in SDL_GAMECONTROLLER_DB.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("#") || !line.ends_with(PLATFORM) {
                continue;
            }

            let mut split = line.split(',');
            if split.next() != Some(guid) {
                continue;
            }

            res.name = split.next().unwrap().trim().to_string();
            res.layout = ControllerLayout::by_name(&res.name);

            for tok in split {
                let mut sep = tok.trim().split(":");
                let ptr = match sep.next().map(|x| x.trim()) {
                    Some("a") => &mut res.a,
                    Some("b") => &mut res.b,
                    Some("x") => &mut res.x,
                    Some("y") => &mut res.y,
                    Some("dpdown") => &mut res.dpad.down,
                    Some("dpleft") => &mut res.dpad.left,
                    Some("dpright") => &mut res.dpad.right,
                    Some("dpup") => &mut res.dpad.up,
                    Some("leftshoulder") => &mut res.l1,
                    Some("lefttrigger") => &mut res.l2,
                    Some("leftstick") => &mut res.l3,
                    Some("rightshoulder") => &mut res.r1,
                    Some("righttrigger") => &mut res.r2,
                    Some("rightstick") => &mut res.r3,
                    Some("leftx") => &mut res.left_stick.0,
                    Some("lefty") => &mut res.left_stick.1,
                    Some("rightx") => &mut res.right_stick.0,
                    Some("righty") => &mut res.right_stick.1,
                    Some("start") => &mut res.start,
                    Some("back") => &mut res.select,
                    _ => continue,
                };

                let val: i32 = {
                    let str_val = match sep.next() {
                        Some(x) => x,
                        None => continue,
                    }.trim();

                    if let Some(str_val) = str_val.strip_prefix("b") {
                        str_val.trim().parse::<i32>().unwrap()
                    } else if let Some(str_val) = str_val.strip_prefix("h0.") {
                        str_val.trim().parse::<i32>().unwrap()
                    } else if let Some(str_val) = str_val.strip_prefix("a") {
                        str_val.trim().parse::<i32>().unwrap()
                    } else {
                        continue;
                    }
                };

                *ptr = val;
            }

            return Some(res);
        }

        return None;
    }
    
    pub fn layout(&self) -> ControllerLayout {
        self.layout
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn a(&self) -> i32 {
        self.a
    }
    
    pub fn b(&self) -> i32 {
        self.b
    }
    
    pub fn x(&self) -> i32 {
        self.x
    }
    
    pub fn y(&self) -> i32 {
        self.y
    }
    
    pub fn dpad(&self) -> &DirMappings {
        &self.dpad
    }
    
    pub fn left_stick(&self) -> ivec2 {
        self.left_stick
    }
    
    pub fn right_stick(&self) -> ivec2 {
        self.right_stick
    }
    
    pub fn l1(&self) -> i32 {
        self.l1
    }
    
    pub fn l2(&self) -> i32 {
        self.l2
    }
    
    pub fn l3(&self) -> i32 {
        self.l3
    }
    
    pub fn r1(&self) -> i32 {
        self.r1
    }
    
    pub fn r2(&self) -> i32 {
        self.r2
    }
    
    pub fn r3(&self) -> i32 {
        self.r3
    }
    
    pub fn start(&self) -> i32 {
        self.start
    }

    pub fn select(&self) -> i32 {
        self.select
    }
}

impl Default for ControllerMappings {
    fn default() -> Self {
        Self {
            layout: ControllerLayout::Xbox, name: String::new(), a: -1, b: -1, x: -1, y: -1, dpad: Default::default(), left_stick: ivec2(-1, -1), right_stick: ivec2(-1, -1), l1: -1, l2: -1, l3: -1, r1: -1, r2: -1, r3: -1, start: -1, select: -1
        }
    }
}