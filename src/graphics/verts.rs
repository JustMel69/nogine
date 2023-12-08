use crate::assert_expr;

use super::super::gl_call;


const F32_SIZE: usize = std::mem::size_of::<f32>();
pub fn set_vertex_attribs(counts: &[usize]) {
    let stride: i32 = (counts.iter().sum::<usize>() * F32_SIZE) as i32;
    assert_expr!(stride != 0, "Stride must be greater than 0");

    let mut offset: i32 = 0;

    for (i, c) in counts.iter().enumerate() {
        gl_call!(gl::VertexAttribPointer(i as u32, *c as i32, gl::FLOAT, gl::FALSE, stride, offset as *const std::ffi::c_void));
        gl_call!(gl::EnableVertexAttribArray(i as u32));

        offset += (c * F32_SIZE) as i32;
    }
}