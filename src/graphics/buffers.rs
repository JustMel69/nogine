use gl::types as gltyp;

use super::super::gl_call;

pub struct GlBuffer {
    id: gltyp::GLuint,
    kind: gltyp::GLenum,
}

impl GlBuffer {
    pub fn new_vbo() -> Self {
        let mut id = 0;
        gl_call!(gl::GenBuffers(1, &mut id));
        assert!(id != 0, "Invalid buffer");

        Self { id, kind: gl::ARRAY_BUFFER }
    }

    pub fn new_ebo() -> Self {
        let mut id = 0;
        gl_call!(gl::GenBuffers(1, &mut id));
        assert!(id != 0, "Invalid buffer");

        Self { id, kind: gl::ELEMENT_ARRAY_BUFFER }
    }

    pub fn set_data_from_slice<T>(&self, data: &[T]) {
        self.bind();
        gl_call!(gl::BufferData(self.kind, (std::mem::size_of::<T>() * data.len()) as isize, data.as_ptr() as *const std::ffi::c_void, gl::STATIC_DRAW));
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


pub struct GlVAO {
    id: gltyp::GLuint,
}

impl GlVAO {
    pub fn new() -> Self {
        let mut id = 0;
        gl_call!(gl::GenVertexArrays(1, &mut id));
        return Self { id };
    }

    pub fn bind(&self) {
        gl_call!(gl::BindVertexArray(self.id));
    }
}

impl Drop for GlVAO {
    fn drop(&mut self) {
        gl_call!(gl::DeleteVertexArrays(1, &self.id));
    }
}