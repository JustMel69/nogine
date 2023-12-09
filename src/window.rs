use std::{sync::mpsc::Receiver, time::Instant};

use glfw::Context as GlfwContext;
use thiserror::Error;

use crate::{Res, input::Input, graphics::{Graphics, RenderStats, pipeline::{RenderPipeline, DefaultRenderPipeline}}, audio::Audio, logging::Logger, log_info, assert_expr, unwrap_opt};

use super::gl_call;

#[derive(Debug, Error)]
pub enum WindowError {
    #[error("{0}")]
    InitError(#[from] glfw::InitError),
    #[error("Couldn't create window")]
    CreationFailure,
}

pub enum WindowMode {
    Fullscreen,
    Windowed,
}

pub struct WindowCfg<'a> {
    pub res: (u32, u32),
    pub title: &'a str,
    pub mode: WindowMode,
    pub main: bool,
}

impl<'a> WindowCfg<'a> {
    pub fn res(mut self, val: impl Into<(u32, u32)>) -> Self {
        self.res = val.into();
        return self;
    }

    pub fn title(mut self, val: &'a str) -> Self {
        self.title = val;
        return self;
    }

    pub fn mode(mut self, val: WindowMode) -> Self {
        self.mode = val;
        return self;
    }

    pub fn init(self) -> Res<Window, WindowError> {
        Logger::init();
        
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).map_err(|e| WindowError::from(e))?;
        
        let monitor = glfw::Monitor::from_primary();
        let mode = match self.mode {
            WindowMode::Fullscreen => glfw::WindowMode::FullScreen(&monitor),
            WindowMode::Windowed => glfw::WindowMode::Windowed,
        };

        let (mut window, events) = glfw.create_window(self.res.0, self.res.1, self.title, mode).ok_or(WindowError::CreationFailure)?;
        window.set_all_polling(true);
        window.make_current();

        gl::load_with(|x| window.get_proc_address(x) as *const _);
        gl_call!(gl::Viewport(0, 0, self.res.0 as i32, self.res.1 as i32));
        
        Graphics::init();
        Audio::init();

        log_info!("Window initialized.");
        return Ok(Window { window, events, glfw, def_res: self.res });
    }
}

impl<'a> Default for WindowCfg<'a> {
    fn default() -> Self {
        Self { res: (1280, 720), title: "Nogine Window", mode: WindowMode::Windowed, main: false }
    }
}





pub struct Window {
    window: glfw::Window,
    events: Receiver<(f64, glfw::WindowEvent)>,
    glfw: glfw::Glfw,
    def_res: (u32, u32),
}

impl Window {
    #[inline]
    pub fn is_running(&self) -> bool {
        !self.window.should_close()
    }

    fn handle_events(&mut self) {
        self.glfw.poll_events();

        for (_, ev) in glfw::flush_messages(&self.events) {
            Input::push_input(ev);
        }
        
    }

    pub fn pre_tick(&mut self, pipeline: Option<&dyn RenderPipeline>) -> RenderStats {
        let mut def_pipeline = DefaultRenderPipeline;
        let pipeline = pipeline.unwrap_or(&mut def_pipeline);
        
        let stats = Graphics::render(pipeline, self.get_size(), self);
        
        Graphics::tick();
        return stats;
    }
    
    pub fn post_tick(&mut self) {
        Graphics::finalize_batch();

        Input::flush();
        self.handle_events();
    }

    #[inline]
    pub fn close(self) {
        self.window.close()
    }

    #[inline]
    pub fn focus(&mut self) {
        self.window.focus()
    }

    #[inline]
    pub fn set_resizable(&mut self, resizable: bool) {
        self.window.set_resizable(resizable)
    }

    #[inline]
    pub fn get_size(&self) -> (u32, u32) {
        let x = self.window.get_size();
        return (x.0 as u32, x.1 as u32)
    }

    #[inline]
    pub fn set_size(&mut self, size: (u32, u32)) {
        self.window.set_size(size.0 as i32, size.1 as i32);
    }

    #[inline]
    pub fn set_aspect_ratio(&mut self, n: u32, d: u32) {
        self.window.set_aspect_ratio(n, d)
    }

    #[inline]
    pub fn aspect_ratio(&self) -> f32 {
        let (w, h) = self.get_size();
        return (w as f32) / (h as f32);
    }

    #[inline]
    pub fn request_attention(&mut self) {
        self.window.request_attention()
    }

    #[inline]
    pub fn set_title(&mut self, title: &str) {
        self.window.set_title(title)
    }

    #[inline]
    pub fn is_maximized(&self) -> bool {
        self.window.is_maximized()
    }

    #[inline]
    pub fn is_minimized(&self) -> bool {
        self.window.is_iconified()
    }

    #[inline]
    pub fn maximize(&mut self) {
        self.window.maximize()
    }

    #[inline]
    pub fn minimize(&mut self) {
        self.window.iconify()
    }

    #[inline]
    pub fn set_window_mode(&mut self, mode: WindowMode) {
        let monitor = glfw::Monitor::from_primary();
        let video_mode = unwrap_opt!(monitor.get_video_mode(), "Couldn't retrieve monitor video mode");

        match mode {
            WindowMode::Fullscreen => self.window.set_monitor(glfw::WindowMode::FullScreen(&monitor), 0, 0, video_mode.width, video_mode.height, Some(video_mode.refresh_rate)),
            WindowMode::Windowed => {
                let res = self.def_res;

                let x = video_mode.width / 2 - res.0 / 2;
                let y = video_mode.height / 2 - res.1 / 2;
                self.window.set_monitor(glfw::WindowMode::Windowed, x as i32, y as i32, res.0, res.1, None)
            },
        }

        //self.window.set_monitor(mode, xpos, ypos, width, height, refresh_rate)
    }

    #[inline]
    pub(crate) fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }

    pub fn force_framerate(&self, last_frame: Instant, target_framerate: f64) {
        assert_expr!(target_framerate > 0.0, "Target framerate must be greater than 0");
        
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