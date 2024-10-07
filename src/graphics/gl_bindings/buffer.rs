use crate::gl_call;

use super::{gl_enum, gl_uint};

#[allow(unused)]
#[repr(u32)] pub enum GlBufferKind {
    VBO = gl::ARRAY_BUFFER,
    EBO = gl::ELEMENT_ARRAY_BUFFER,
    UBO = gl::UNIFORM_BUFFER,
}

impl Into<gl_enum> for GlBufferKind {
    fn into(self) -> gl_enum {
        return unsafe { std::mem::transmute(self) }
    }
}

#[allow(unused)]
#[repr(u32)] pub enum GlBufferUsage {
    StreamDraw = gl::STREAM_DRAW,
    StreamRead = gl::STREAM_READ,
    StreamCopy = gl::STREAM_COPY,

    StaticDraw = gl::STATIC_DRAW,
    StaticRead = gl::STATIC_READ,
    StaticCopy = gl::STATIC_COPY,

    DynamicDraw = gl::DYNAMIC_DRAW,
    DynamicRead = gl::DYNAMIC_READ,
    DynamicCopy = gl::DYNAMIC_COPY,
}

impl Into<gl_enum> for GlBufferUsage {
    fn into(self) -> gl_enum {
        return unsafe { std::mem::transmute(self) }
    }
}


pub struct GlBuffer {
    id: gl_uint,
    kind: gl_enum,
}

impl GlBuffer {
    #[allow(unused)]
    pub fn prealloc(size: usize, kind: GlBufferKind, usage: GlBufferUsage) -> Self {
        let buf = Self::empty(kind);
        let usage = usage.into();

        gl_call!(gl::BindBuffer(buf.kind, buf.id));
        gl_call!(gl::BufferData(buf.kind, size as isize, std::ptr::null(), usage));

        return buf;
    }

    pub fn new<T>(data: &[T], kind: GlBufferKind, usage: GlBufferUsage) -> Self {
        let buf = Self::empty(kind);
        buf.set(data, usage);
        return buf;
    }

    fn empty(kind: GlBufferKind) -> Self {
        let kind = kind.into();
        
        let mut id = 0;
        gl_call!(gl::GenBuffers(1, &mut id));
        assert_expr!(id != 0);

        return Self { id, kind };
    }

    pub fn set<T>(&self, data: &[T], usage: GlBufferUsage) {
        let usage = usage.into();

        gl_call!(gl::BindBuffer(self.kind, self.id));
        gl_call!(gl::BufferData(self.kind, (std::mem::size_of::<T>() * data.len()) as isize, data.as_ptr() as *const std::ffi::c_void, usage));
    }

    #[allow(unused)]
    pub fn subdata<T>(&self, data: &[T], byte_offset: usize) {
        gl_call!(gl::BindBuffer(self.kind, self.id));
        gl_call!(gl::BufferSubData(self.kind, byte_offset as isize, (std::mem::size_of::<T>() * data.len()) as isize, data.as_ptr() as *const std::ffi::c_void));
    }

    pub fn bind(&self) {
        gl_call!(gl::BindBuffer(self.kind, self.id));
    }
}

impl Drop for GlBuffer {
    fn drop(&mut self) {
        gl_call!(gl::DeleteBuffers(1, &self.id));
    }
}
