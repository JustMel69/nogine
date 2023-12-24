use std::{num::Wrapping, u128, ops::Range};

use crate::unwrap_res;

pub struct RNG {
    seed: u128,
}

static mut DEF_RNG: Option<RNG> = None;


impl RNG {
    const A: Wrapping<u128> = Wrapping(1103515245);
    const C: Wrapping<u128> = Wrapping(42069);
    
    pub fn global() -> &'static mut RNG {
        unsafe {
            if DEF_RNG.is_none() {
                DEF_RNG = Some(RNG::new());
            }

            return DEF_RNG.as_mut().unwrap_unchecked();
        }
    }



    pub fn new() -> Self {
        let seed = unwrap_res!(std::time::UNIX_EPOCH.elapsed()).as_millis();
        return Self { seed };
    }

    pub fn with_seed(seed: u128) -> Self {
        return Self { seed };
    }

    
    
    fn gen_raw(&mut self) -> u128 {
        self.seed = (Wrapping(self.seed) * Self::A + Self::C).0;
        self.seed = self.seed.swap_bytes();
        return self.seed;
    }


    /// Generates a random number. For integers, the value will always be positive and cover the full integer range. For floats, the value will be in the range [0..1)
    pub fn gen<T: Sample>(&mut self) -> T {
        let val = self.gen_raw();
        return T::gen(RNGCore(val));
    }

    /// Generates a random number in the provided range.
    pub fn gen_range<T: SampleRange>(&mut self, range: Range<T>) -> T {
        let val = self.gen_raw();
        return T::gen_range(RNGCore(val), range);
    }

    /// Generates a random integer, positive or negative and covering the full integer range.
    pub fn gen_signed<T: SampleSigned>(&mut self) -> T {
        let val = self.gen_raw();
        return T::gen_signed(RNGCore(val));
    }

    /// Picks a random item in the slice.
    pub fn pick<'a, T>(&mut self, list: &'a [T]) -> &'a T {
        let index = self.gen_range(0..list.len());
        return &list[index];
    }
}

#[repr(transparent)] pub struct RNGCore(u128);

pub trait Sample : Sized {
    fn gen(core: RNGCore) -> Self;
}

pub trait SampleRange : Sample {
    fn gen_range(core: RNGCore, range: Range<Self>) -> Self;
}

pub trait SampleSigned : Sample {
    fn gen_signed(core: RNGCore) -> Self;
}


impl Sample for u128 {
    fn gen(core: RNGCore) -> Self {
        return core.0;
    }
}

impl SampleRange for u128 {
    fn gen_range(core: RNGCore, range: Range<Self>) -> Self {
        return (core.0 - range.start) % (range.end - range.start) + range.start;
    }
}



impl Sample for i128 {
    fn gen(core: RNGCore) -> Self {
        return (core.0 & !(1 << 127)) as i128;
    }
}

impl SampleRange for i128 {
    fn gen_range(core: RNGCore, range: Range<Self>) -> Self {
        return (Self::gen(core) - range.start) % (range.end - range.start) + range.start;
    }
}

impl SampleSigned for i128 {
    fn gen_signed(core: RNGCore) -> Self {
        return unsafe { std::mem::transmute(core.0) };
    }
}



impl Sample for usize {
    fn gen(core: RNGCore) -> Self {
        return (core.0 & usize::MAX as u128) as usize;
    }
}

impl SampleRange for usize {
    fn gen_range(core: RNGCore, range: Range<Self>) -> Self {
        return (Self::gen(core) - range.start) % (range.end - range.start) + range.start;
    }
}



impl Sample for isize {
    fn gen(core: RNGCore) -> Self {
        return (core.0 & isize::MAX as u128) as isize;
    }
}

impl SampleRange for isize {
    fn gen_range(core: RNGCore, range: Range<Self>) -> Self {
        return (Self::gen(core) - range.start) % (range.end - range.start) + range.start;
    }
}

impl SampleSigned for isize {
    fn gen_signed(core: RNGCore) -> Self {
        return unsafe { std::mem::transmute((core.0 & usize::MAX as u128) as usize) };
    }
}



impl Sample for u64 {
    fn gen(core: RNGCore) -> Self {
        return (core.0 & u64::MAX as u128) as u64;
    }
}

impl SampleRange for u64 {
    fn gen_range(core: RNGCore, range: Range<Self>) -> Self {
        return (Self::gen(core) - range.start) % (range.end - range.start) + range.start;
    }
}



impl Sample for i64 {
    fn gen(core: RNGCore) -> Self {
        return (core.0 & i64::MAX as u128) as i64;
    }
}

impl SampleRange for i64 {
    fn gen_range(core: RNGCore, range: Range<Self>) -> Self {
        return (Self::gen(core) - range.start) % (range.end - range.start) + range.start;
    }
}

impl SampleSigned for i64 {
    fn gen_signed(core: RNGCore) -> Self {
        return unsafe { std::mem::transmute((core.0 & u64::MAX as u128) as u64) };
    }
}



impl Sample for u32 {
    fn gen(core: RNGCore) -> Self {
        return (core.0 & u32::MAX as u128) as u32;
    }
}

impl SampleRange for u32 {
    fn gen_range(core: RNGCore, range: Range<Self>) -> Self {
        return (Self::gen(core) - range.start) % (range.end - range.start) + range.start;
    }
}



impl Sample for i32 {
    fn gen(core: RNGCore) -> Self {
        return (core.0 & i32::MAX as u128) as i32;
    }
}

impl SampleRange for i32 {
    fn gen_range(core: RNGCore, range: Range<Self>) -> Self {
        return (Self::gen(core) - range.start) % (range.end - range.start) + range.start;
    }
}

impl SampleSigned for i32 {
    fn gen_signed(core: RNGCore) -> Self {
        return unsafe { std::mem::transmute((core.0 & u32::MAX as u128) as u32) };
    }
}



impl Sample for u16 {
    fn gen(core: RNGCore) -> Self {
        return (core.0 & u16::MAX as u128) as u16;
    }
}

impl SampleRange for u16 {
    fn gen_range(core: RNGCore, range: Range<Self>) -> Self {
        return (Self::gen(core) - range.start) % (range.end - range.start) + range.start;
    }
}



impl Sample for i16 {
    fn gen(core: RNGCore) -> Self {
        return (core.0 & i16::MAX as u128) as i16;
    }
}

impl SampleRange for i16 {
    fn gen_range(core: RNGCore, range: Range<Self>) -> Self {
        return (Self::gen(core) - range.start) % (range.end - range.start) + range.start;
    }
}

impl SampleSigned for i16 {
    fn gen_signed(core: RNGCore) -> Self {
        return unsafe { std::mem::transmute((core.0 & u16::MAX as u128) as u16) };
    }
}



impl Sample for u8 {
    fn gen(core: RNGCore) -> Self {
        return (core.0 & u8::MAX as u128) as u8;
    }
}

impl SampleRange for u8 {
    fn gen_range(core: RNGCore, range: Range<Self>) -> Self {
        return (Self::gen(core) - range.start) % (range.end - range.start) + range.start;
    }
}



impl Sample for i8 {
    fn gen(core: RNGCore) -> Self {
        return (core.0 & i8::MAX as u128) as i8;
    }
}

impl SampleRange for i8 {
    fn gen_range(core: RNGCore, range: Range<Self>) -> Self {
        return (Self::gen(core) - range.start) % (range.end - range.start) + range.start;
    }
}

impl SampleSigned for i8 {
    fn gen_signed(core: RNGCore) -> Self {
        return unsafe { std::mem::transmute((core.0 & u8::MAX as u128) as u8) };
    }
}



impl Sample for bool {
    fn gen(core: RNGCore) -> Self {
        return core.0 % 2 == 1;
    }
}



impl Sample for f64 {
    fn gen(core: RNGCore) -> Self {
        return (core.0 as f64) / (u128::MAX as f64);
    }
}

impl SampleRange for f64 {
    fn gen_range(core: RNGCore, range: Range<Self>) -> Self {
        return Self::gen(core) * (range.end - range.start) + range.start;
    }
}



impl Sample for f32 {
    fn gen(core: RNGCore) -> Self {
        return f64::gen(core) as f32;
    }
}

impl SampleRange for f32 {
    fn gen_range(core: RNGCore, range: Range<Self>) -> Self {
        return Self::gen(core) * (range.end - range.start) + range.start;
    }
}