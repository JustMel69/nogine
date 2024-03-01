use crate::{gl_call, graphics::verts::VertAttrib};

use super::buffer::GlBuffer;

pub struct GlVertexArrayObject {
    id: gl::types::GLenum,
    pub vbo: GlBuffer,
    pub ebo: GlBuffer,
    pub attribs: Box<[VertAttrib]>
}

impl GlVertexArrayObject {
    pub fn new(vbo: GlBuffer, ebo: GlBuffer, attribs: &[VertAttrib]) -> Self {
        let mut id = 0;
        gl_call!(gl::GenVertexArrays(1, &mut id));

        let vao = Self { vbo, ebo, id, attribs: attribs.into() };
        return vao;
    }

    pub fn bind_attribs(&self) {
        gl_call!(gl::BindVertexArray(self.id));

        self.vbo.bind();
        self.ebo.bind();

        let stride = self.attribs.iter().map(|x| x.full_size()).sum::<usize>();
        let mut offset = 0;

        for (i, a) in self.attribs.iter().enumerate() {
            match a {
                VertAttrib::F32 | VertAttrib::Vec2 | VertAttrib::Vec3 | VertAttrib::Vec4 => 
                    gl_call!(gl::VertexAttribPointer(i as u32, a.size() as i32, gl::FLOAT, gl::FALSE, stride as i32, offset as *const std::ffi::c_void)),
            }
            gl_call!(gl::EnableVertexAttribArray(i as u32));
            offset += a.full_size();
        }
    }

    pub fn bind(&self) {
        gl_call!(gl::BindVertexArray(self.id));
    }
}

impl Drop for GlVertexArrayObject {
    fn drop(&mut self) {
        gl_call!(gl::DeleteVertexArrays(1, &self.id));
    }
}