use std::sync::Arc;

use crate::{graphics::verts::set_vertex_attribs, math::Matrix3x3, assert_expr};

use super::{buffers::{GlBuffer, GlVAO}, texture::{TextureCore, Texture}, gl_call, BlendingMode, material::Material};

pub struct RefBatchState<'a> {
    pub material: Material,
    pub attribs: &'a [usize],
    pub textures: &'a [&'a Texture],
    pub blending: BlendingMode,
    pub camera: &'a Matrix3x3,
    pub is_line: bool,
}

impl<'a> Into<BatchState> for RefBatchState<'a> {
    fn into(self) -> BatchState {
        return BatchState::new(
            self.material,
            self.attribs.into(),
            self.textures.iter().map(|x| x.clone_core()).collect(),
            self.blending,
            self.camera.clone(),
            self.is_line
        );
    }
}

#[derive(Debug)]
pub struct BatchState {
    material: Material,
    attribs: Box<[usize]>,
    textures: Box<[Arc<TextureCore>]>,
    blending: BlendingMode,
    camera: Matrix3x3,
    is_line: bool,
}

impl BatchState {
    fn new<'a>(material: Material, attribs: Box<[usize]>, textures: Box<[Arc<TextureCore>]>, blending: BlendingMode, camera: Matrix3x3, is_line: bool) -> Self {
        return Self { material, attribs, textures, blending, is_line, camera };
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
        vbo.set_data_from_slice(&self.verts);

        let ebo = GlBuffer::new_ebo();
        ebo.set_data_from_slice(&self.tris);

        return BatchProduct { vao, vbo, ebo, trilen: self.tris.len() as i32, state: self.state };
    }

    pub fn is_of_state(&self, state: &RefBatchState) -> bool {
        return self.state.attribs.iter().eq(state.attribs.iter()) &&
            self.state.blending == state.blending &&
            self.state.material == state.material &&
            self.state.is_line == state.is_line &&
            &self.state.camera == state.camera &&
            self.state.textures.iter().map(|x| x.as_ref()).eq(state.textures.iter().map(|x| x.core()));
    }
}


// Is produced in post-tick, rendered in pre-tick
pub struct BatchProduct {
    vao: GlVAO,
    vbo: GlBuffer,
    ebo: GlBuffer,
    trilen: i32,
    state: BatchState,
}

impl BatchProduct {
    pub fn render(&self) {
        self.vao.bind();
        self.vbo.bind();
        self.ebo.bind();

        set_vertex_attribs(&self.state.attribs);

        for (i, t) in self.state.textures.iter().enumerate() {
            t.enable(i as u8);
        }

        self.state.material.enable();

        let tf_mat_address = gl_call!(gl::GetUniformLocation(self.state.material.shader().id(), b"mvm\0".as_ptr() as *const i8));
        assert_expr!(tf_mat_address != -1, "Can't find 'mvm' uniform in shader.");
        gl_call!(gl::UniformMatrix3fv(tf_mat_address, 1, gl::TRUE, self.state.camera.ptr()));

        self.state.blending.apply();

        gl_call!(gl::DrawElements(
            if self.state.is_line { gl::LINES } else { gl::TRIANGLES },
            self.trilen,
            gl::UNSIGNED_INT,
            std::ptr::null()
        ));
    }
}


pub(super) struct TargetBatchData {
    pub curr_batch: Option<BatchMesh>,
    pub ready_batches: Vec<BatchProduct>,
    pub render_batches: Vec<BatchProduct>,
}

impl TargetBatchData {
    const fn new() -> Self {
        return Self { curr_batch: None, ready_batches: Vec::new(), render_batches: Vec::new() }
    }
}

pub(super) struct BatchData {
    pub targets: [Option<TargetBatchData>; 256],
}

impl BatchData {
    pub fn new() -> Self {
        return Self {
            targets: std::array::from_fn(|_| None),
        }
    }

    pub fn send(&mut self, target_id: u8, state: RefBatchState<'_>, verts: &[f32], tris: &[u32]) {
        self.check_state(target_id, &state);

        let target = self.realize_target(target_id);
        
        if target.curr_batch.is_none() {
            target.curr_batch = Some(BatchMesh::new(state.into()));
        }

        target.curr_batch.as_mut().unwrap().push(verts, tris);

    }

    pub fn check_state(&mut self, target_id: u8, state: &RefBatchState<'_>) {
        if let Some(target) = &mut self.targets[target_id as usize] {
            if target.curr_batch.is_none() {
                return;
            }
            
            if !target.curr_batch.as_ref().unwrap().is_of_state(&state) {
                self.finalize_batch(target_id);
            }
        }
    }

    pub fn finalize_batch(&mut self, target_id: u8) {
        if let Some(target) = &mut self.targets[target_id as usize] {    
            let mut batch: Option<BatchMesh> = None;
            std::mem::swap(&mut batch, &mut target.curr_batch);
            
            if let Some(x) = batch {
                let product = x.consume();
                target.ready_batches.push(product);
            }
        }
    }

    pub fn swap_batch_buffers(&mut self, target_id: u8) {
        if let Some(target) = &mut self.targets[target_id as usize] {
            std::mem::swap(&mut target.ready_batches, &mut target.render_batches);
            target.ready_batches.clear();
        }
    }

    fn realize_target(&mut self, target: u8) -> &mut TargetBatchData {
        if self.targets[target as usize].is_none() {
            self.targets[target as usize] = Some(TargetBatchData::new());
        }

        return self.targets[target as usize].as_mut().unwrap();
    }
}