use std::ffi::CString;

use crate::{graphics::{verts::set_vertex_attribs, Graphics}, math::Matrix3x3};

use super::{shader::Shader, uniforms::Uniform, buffers::{GlBuffer, GlVAO}, texture::TextureCore, gl_call, BlendingMode};

pub type PreUniformVec = Vec<(CString, Uniform)>;
pub type UniformVec = Vec<(i32, Uniform)>;

pub struct BatchMesh {
    verts: Vec<f32>,
    tris: Vec<u32>,
    shader: Shader,
    uniforms: UniformVec,
    attribs: Box<[usize]>,
    textures: Vec<TextureCore>,
    blending: BlendingMode,
}

impl BatchMesh {
    pub fn new(shader: Shader, uniforms: PreUniformVec, attribs: &[usize], textures: Vec<TextureCore>, blending: BlendingMode) -> Self {
        let uniforms = uniforms.into_iter().map(|x| {
            let pos = gl_call!(gl::GetUniformLocation(shader.id(), x.0.as_ptr()));
            (pos, x.1)
        }).collect::<Vec<_>>();
        
        return Self { verts: Vec::new(), tris: Vec::new(), shader, uniforms, attribs: attribs.into(), textures, blending };
    }
    
    pub fn push(&mut self, verts: &[f32], tris: &[u32]) {
        let attrib_len: usize = self.attribs.iter().sum();
        let voffset = self.verts.len() / attrib_len;
        self.verts.extend_from_slice(verts);
        self.tris.extend(tris.iter().map(|x| *x + voffset as u32));
    }

    pub fn consume(self) -> BatchProduct {
        let vao = GlVAO::new();
        vao.bind();
        
        let vbo = GlBuffer::new_vbo();
        vbo.set_data(self.verts.as_slice());

        let ebo = GlBuffer::new_ebo();
        ebo.set_data(self.tris.as_slice());

        return BatchProduct { vbo, ebo, vao, shader: self.shader, uniforms: self.uniforms, attribs: self.attribs, trilen: self.tris.len() as i32, textures: self.textures, blending: self.blending };
    }
}


// Is produced in post-tick, rendered in pre-tick
pub struct BatchProduct {
    vbo: GlBuffer,
    ebo: GlBuffer,
    vao: GlVAO,
    shader: Shader,
    uniforms: UniformVec,
    attribs: Box<[usize]>,
    trilen: i32,
    textures: Vec<TextureCore>,
    blending: BlendingMode,
}

impl BatchProduct {
    pub fn render(&self) {
        self.shader.enable();

        for (l, u) in &self.uniforms {
            match u {
                Uniform::Float(x) => gl_call!(gl::Uniform1f(*l, *x)),
                Uniform::Float2(x, y) => gl_call!(gl::Uniform2f(*l, *x, *y)),
                Uniform::Float3(x, y, z) => gl_call!(gl::Uniform3f(*l, *x, *y, *z)),
                Uniform::Float4(x, y, z, w) => gl_call!(gl::Uniform4f(*l, *x, *y, *z ,*w)),
                Uniform::Int(x) => gl_call!(gl::Uniform1i(*l, *x)),
                Uniform::Uint(x) => gl_call!(gl::Uniform1ui(*l, *x)),
            }
        }

        match self.blending {
            BlendingMode::AlphaMix => gl_call!(gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA)),
            BlendingMode::Additive => gl_call!(gl::BlendFunc(gl::SRC_ALPHA, gl::ONE)),
            BlendingMode::Multiplicative => gl_call!(gl::BlendFunc(gl::DST_COLOR, gl::ZERO)),
        }
        
        for (i, t) in self.textures.iter().enumerate() {
            t.enable(i as u8);
        }

        self.vao.bind();

        set_vertex_attribs(&self.attribs);

        Graphics::set_tf_mat(Matrix3x3::IDENTITY);
        gl_call!(gl::DrawElements(gl::TRIANGLES, self.trilen, gl::UNSIGNED_INT, std::ptr::null()));
    }
}