use std::sync::mpsc::Receiver;

use glfw::Context as GlfwContext;
use thiserror::Error;

use crate::{Res, context::Context, input::Input};

#[repr(transparent)]
pub struct Monitor<'a>(&'a glfw::Monitor);

#[derive(Debug, Error)]
pub enum WindowError {
    #[error("Couldn't create window")]
    CreationFailure,
}

pub enum WindowMode<'a> {
    Fullscreen(Monitor<'a>),
    Windowed,
}

impl<'a> Into<glfw::WindowMode<'a>> for WindowMode<'a> {
    fn into(self) -> glfw::WindowMode<'a> {
        unsafe { std::mem::transmute::<Self, glfw::WindowMode>(self) }
    }
}

pub struct WindowCfg<'a> {
    pub res: (u32, u32),
    pub title: &'a str,
    pub mode: WindowMode<'a>,
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

    pub fn mode(mut self, val: WindowMode<'a>) -> Self {
        self.mode = val;
        return self;
    }

    pub fn main(mut self, val: bool) -> Self {
        self.main = val;
        return self;
    }

    pub fn init(self, ctx: &mut Context) -> Res<Window, WindowError> {
        let (mut window, events) = ctx.glfw.create_window(self.res.0, self.res.1, self.title, self.mode.into()).ok_or(WindowError::CreationFailure)?;
        window.set_key_polling(true);
        window.make_current();

        return Ok(Window { window, events, main: self.main });
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
    main: bool,
}

impl Window {
    #[inline]
    pub fn is_running(&self) -> bool {
        !self.window.should_close()
    }

    pub fn handle_events(&mut self, ctx: &mut Context) {
        ctx.glfw.poll_events();

        for (_, ev) in glfw::flush_messages(&self.events) {
            Input::push_input(ev, self.main);
        }
        
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
    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }
}