use std::time::Instant;

pub struct Context {
    pub(crate) glfw: glfw::Glfw,
}

impl Context {
    pub fn init() -> Self {
        Self { glfw: glfw::init(glfw::FAIL_ON_ERRORS).unwrap() }
    }

    pub fn force_framerate(&self, last_frame: Instant, target_framerate: f64) {
        assert!(target_framerate > 0.0);
        
        let target_ts = 1.0 / target_framerate;

        loop {
            let ts = last_frame.elapsed().as_secs_f64();
            if ts > target_ts {
                return;
            }
        }
    }

    pub fn set_vsync(&mut self, vsync: bool) {
        self.glfw.set_swap_interval(glfw::SwapInterval::Sync(if vsync { 1 } else { 0 }))
    }
}