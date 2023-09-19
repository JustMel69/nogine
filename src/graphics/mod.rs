use std::sync::RwLock;

use crate::{math::{Vector2, Matrix3x3}, color::Color4, graphics::buffers::{GlBuffer, GlVAO}};

use self::shader::{Shader, SubShader, SubShaderType};

use super::gl_call;

pub mod shader;
mod buffers;


static GRAPHICS: RwLock<Graphics> = RwLock::new(Graphics::new());


enum Mode {
    Unset,
    Rect
}

const DEF_RECT_VERT_SHADER: &str = r#"
#version 330 core
layout (location = 0) in vec2 vPos;
layout (location = 1) in vec4 vCol;

out vec4 fCol;

uniform mat3 tf_mat;
uniform mat3 cam_mat;

void main() {
    gl_Position = vec4((cam_mat * tf_mat * vec3(vPos, 1.0)).xy, 0.0, 1.0);
    fCol = vCol;
}
"#;

const DEF_RECT_FRAG_SHADER: &str = r#"
#version 330 core

in vec4 fCol;
out vec4 FragColor;

void main() {
    FragColor = fCol;
}
"#;



pub struct Graphics {
    mode: Mode,
    def_rect_shader: Shader,
    scheduled_cam_mat: Matrix3x3,
}

impl Graphics {
    const fn new() -> Self {
        Self { mode: Mode::Unset, def_rect_shader: Shader::invalid(), scheduled_cam_mat: Matrix3x3::IDENTITY }
    }

    pub(crate) fn init() {
        let mut writer = GRAPHICS.write().unwrap();
        writer.def_rect_shader = Shader::new(
            SubShader::new(&DEF_RECT_VERT_SHADER, SubShaderType::Vert),
            SubShader::new(&DEF_RECT_FRAG_SHADER, SubShaderType::Frag)
        );
    }

    pub(crate) fn frame_start() {
        Self::update_cam_mat();
    }
    
    pub fn draw_rect(left_down: Vector2, extents: Vector2, color: Color4) {
        Self::draw_rect_full(left_down, extents, 0.0, [color; 4])
    }
    
    const RECT_TRIS: [u16; 6] = [0, 1, 2, 2, 3, 0];
    pub fn draw_rect_full(left_down: Vector2, extents: Vector2, rot: f32, colors: [Color4; 4]) {
        #[repr(C)]
        struct Vert(Vector2, Color4);
    
        let vao = GlVAO::new();
        vao.bind();

        let verts = [Vert(Vector2::ZERO, colors[0]), Vert(Vector2::UP, colors[1]), Vert(Vector2::ONE, colors[2]), Vert(Vector2::RIGHT, colors[3])];
        let vert_buffer = GlBuffer::new(gl::ARRAY_BUFFER);
        vert_buffer.set_data(&verts);
    
        Self::setup_rect_mode();
        Self::set_tf_mat(Matrix3x3::transform_matrix(left_down, rot, extents));
    
        let tri_buffer = GlBuffer::new(gl::ELEMENT_ARRAY_BUFFER);
        tri_buffer.set_data(&Self::RECT_TRIS);
        
        vao.bind();
        gl_call!(gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_SHORT, 0 as *const std::ffi::c_void));
    }

    pub fn set_cam(pos: Vector2, rot: f32, size: Vector2) {
        assert!(size.0 != 0.0 && size.1 != 0.0);
        let mat = Matrix3x3::inv_transform_matrix(pos, rot, size);

        let mut writer = GRAPHICS.write().unwrap();
        writer.scheduled_cam_mat = mat;
    }

    fn setup_rect_mode() {
        let mut writer = GRAPHICS.write().unwrap();
        
        if matches!(writer.mode, Mode::Rect) {
            return;
        }

        const F32_SIZE: i32 = std::mem::size_of::<f32>() as i32;
        gl_call!(gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 6 * F32_SIZE, 0 as *const std::ffi::c_void));
        gl_call!(gl::EnableVertexAttribArray(0));
        gl_call!(gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, 6 * F32_SIZE, (2 * F32_SIZE) as *const std::ffi::c_void));
        gl_call!(gl::EnableVertexAttribArray(1));

        writer.def_rect_shader.enable();

        writer.mode = Mode::Rect;
    }

    const TF_MAT_NAME: &str = "tf_mat\0";
    fn set_tf_mat(mat: Matrix3x3) {
        let reader = GRAPHICS.read().unwrap();
        let shader = match reader.get_current_shader() {
            Some(x) => x,
            None => return,
        };

        let tf_mat_address = gl_call!(gl::GetUniformLocation(shader.id(), Self::TF_MAT_NAME.as_ptr() as *const i8));
        assert!(tf_mat_address != -1);
        
        gl_call!(gl::UniformMatrix3fv(tf_mat_address, 1, gl::FALSE, mat.ptr()))
    }

    const CAM_MAT_NAME: &str = "cam_mat\0";
    fn update_cam_mat() {
        let reader = GRAPHICS.read().unwrap();
        let shader = match reader.get_current_shader() {
            Some(x) => x,
            None => return,
        };

        let cam_mat_address = gl_call!(gl::GetUniformLocation(shader.id(), Self::CAM_MAT_NAME.as_ptr() as *const i8));
        assert!(cam_mat_address != -1);
        
        gl_call!(gl::UniformMatrix3fv(cam_mat_address, 1, gl::FALSE, reader.scheduled_cam_mat.ptr()))
    }

    fn get_current_shader(&self) -> Option<&Shader> {
        match self.mode {
            Mode::Unset => None,
            Mode::Rect => Some(&self.def_rect_shader),
        }
    }
}