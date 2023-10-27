use crate::{graphics::{verts::set_vertex_attribs, Graphics}, math::Matrix3x3};

use super::{shader::Shader, uniforms::Uniform, buffers::{GlBuffer, GlVAO}, texture::{TextureCore, Texture}, gl_call, BlendingMode};

pub type UniformSlice<'a> = &'a [(&'a [u8], Uniform)];
pub type UniformVec = Vec<(i32, Uniform)>;

pub struct RefBatchState<'a> {
    pub shader: &'a Shader,
    pub uniforms: UniformSlice<'a>,
    pub attribs: &'a [usize],
    pub textures: &'a [&'a Texture],
    pub blending: BlendingMode,
}

impl<'a> Into<BatchState> for RefBatchState<'a> {
    fn into(self) -> BatchState {
        return BatchState::new(
            self.shader.clone(),
            self.uniforms,
            self.attribs.into(),
            self.textures.iter().map(|x| x.core().clone()).collect(),
            self.blending
        );
    }
}

pub struct BatchState {
    shader: Shader,
    uniforms: UniformVec,
    attribs: Box<[usize]>,
    textures: Box<[TextureCore]>,
    blending: BlendingMode,
}

impl BatchState {
    pub fn new<'a>(shader: Shader, uniforms: UniformSlice<'a>, attribs: Box<[usize]>, textures: Box<[TextureCore]>, blending: BlendingMode) -> Self {
        let uniforms = uniforms.into_iter().map(|x| {
            let pos = gl_call!(gl::GetUniformLocation(shader.id(), x.0.as_ptr() as *const i8));
            (pos, x.1)
        }).collect::<Vec<_>>();

        return Self { shader, uniforms, attribs, textures, blending };
    }
}


pub struct BatchMesh {
    verts: Vec<f32>,
    tris: Vec<u32>,
    state: BatchState,
}

impl BatchMesh {
    pub fn new(state: BatchState) -> Self {
        return Self { verts: Vec::new(), tris: Vec::new(), state };
    }
    
    pub fn push(&mut self, verts: &[f32], tris: &[u32]) {
        let attrib_len: usize = self.state.attribs.iter().sum();
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

        return BatchProduct { vbo, ebo, vao, trilen: self.tris.len() as i32, state: self.state };
    }
}


// Is produced in post-tick, rendered in pre-tick
pub struct BatchProduct {
    vbo: GlBuffer,
    ebo: GlBuffer,
    vao: GlVAO,
    trilen: i32,
    state: BatchState,
}

impl BatchProduct {
    pub fn render(&self) {
        self.state.shader.enable();

        for (l, u) in &self.state.uniforms {
            match u {
                Uniform::Float(x) => gl_call!(gl::Uniform1f(*l, *x)),
                Uniform::Float2(x, y) => gl_call!(gl::Uniform2f(*l, *x, *y)),
                Uniform::Float3(x, y, z) => gl_call!(gl::Uniform3f(*l, *x, *y, *z)),
                Uniform::Float4(x, y, z, w) => gl_call!(gl::Uniform4f(*l, *x, *y, *z ,*w)),
                Uniform::Int(x) => gl_call!(gl::Uniform1i(*l, *x)),
                Uniform::Uint(x) => gl_call!(gl::Uniform1ui(*l, *x)),
            }
        }

        match self.state.blending {
            BlendingMode::AlphaMix => gl_call!(gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA)),
            BlendingMode::Additive => gl_call!(gl::BlendFunc(gl::SRC_ALPHA, gl::ONE)),
            BlendingMode::Multiplicative => gl_call!(gl::BlendFunc(gl::DST_COLOR, gl::ZERO)),
        }
        
        for (i, t) in self.state.textures.iter().enumerate() {
            t.enable(i as u8);
        }

        self.vao.bind();

        set_vertex_attribs(&self.state.attribs);

        Graphics::set_tf_mat(Matrix3x3::IDENTITY);
        gl_call!(gl::DrawElements(gl::TRIANGLES, self.trilen, gl::UNSIGNED_INT, std::ptr::null()));
    }
}