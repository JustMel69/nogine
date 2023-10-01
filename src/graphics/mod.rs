use std::sync::RwLock;

use crate::{math::{Vector2, Matrix3x3}, color::Color4, graphics::{buffers::{GlBuffer, GlVAO}, verts::set_vertex_attribs}};

use self::shader::{Shader, SubShader, SubShaderType};

use super::gl_call;

pub mod shader;
pub mod buffers;
pub mod verts;


static GRAPHICS: RwLock<Graphics> = RwLock::new(Graphics::new());

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Unset,
    Rect,
    _Custom,
}

impl Mode {
    pub fn matches(&self, other: &Self) -> bool {
        if matches!(self, Mode::_Custom) || matches!(other, Mode::_Custom) {
            return false;
        }

        return self == other;
    }
}


const DEF_RECT_VERT_SHADER: &str = include_str!("../inline/def_rect_vert_shader.glsl");
const DEF_RECT_FRAG_SHADER: &str = include_str!("../inline/def_rect_frag_shader.glsl");

const DEFAULT_CAM_DATA: CamData = CamData { pos: Vector2::ZERO, height: 1.0 };
struct CamData {
    pos: Vector2,
    height: f32,
}

pub struct Graphics {
    mode: Mode,
    def_rect_shader: Shader,
    scheduled_cam_data: CamData,
    curr_cam_mat: Matrix3x3,
}

impl Graphics {
    const fn new() -> Self {
        Self { mode: Mode::Unset, def_rect_shader: Shader::invalid(), scheduled_cam_data: DEFAULT_CAM_DATA, curr_cam_mat: Matrix3x3::IDENTITY }
    }

    pub(crate) fn init() {
        let mut writer = GRAPHICS.write().unwrap();
        writer.def_rect_shader = Shader::new(
            SubShader::new(&DEF_RECT_VERT_SHADER, SubShaderType::Vert),
            SubShader::new(&DEF_RECT_FRAG_SHADER, SubShaderType::Frag)
        );
    }

    pub fn tick(aspect_ratio: f32) {
        // Update camera matrix
        let mut writer = GRAPHICS.write().unwrap();
        
        let cam_data = &writer.scheduled_cam_data;
        let size = Vector2(cam_data.height * aspect_ratio, cam_data.height);
        //println!("{size:?}");
        writer.curr_cam_mat = Matrix3x3::cam_matrix(cam_data.pos, size);
    }
    
    pub fn draw_rect(left_down: Vector2, extents: Vector2, color: Color4) {
        Self::draw_rect_full(left_down, extents, 0.0, [color; 4])
    }
    
    const RECT_TRIS: [u16; 6] = [0, 1, 2, 2, 3, 0];
    pub fn draw_rect_full(left_down: Vector2, extents: Vector2, rot: f32, colors: [Color4; 4]) {
        #[repr(C)]
        struct Vert(Vector2, Color4);
    
        Self::change_mode(Mode::Rect);

        let vert_data = [Vert(Vector2::ZERO, colors[0]), Vert(Vector2::UP, colors[1]), Vert(Vector2::ONE, colors[2]), Vert(Vector2::RIGHT, colors[3])];

        let vao = GlVAO::new();
        vao.bind();
        
        let vbo = GlBuffer::new(gl::ARRAY_BUFFER);
        vbo.set_data(&vert_data);

        let ebo = GlBuffer::new(gl::ELEMENT_ARRAY_BUFFER);
        ebo.set_data(&Self::RECT_TRIS);

        set_vertex_attribs(&[2, 4]);
        Self::set_tf_mat(Matrix3x3::transform_matrix(left_down, rot, extents));

        vao.bind();
        gl_call!(gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_SHORT, std::ptr::null()));
    }

    pub fn set_cam(pos: Vector2, height: f32) {
        assert!(height != 0.0);

        let mut writer = GRAPHICS.write().unwrap();
        writer.scheduled_cam_data = CamData { pos, height };
    }

    fn change_mode(mode: Mode) {
        let mut writer = GRAPHICS.write().unwrap();
        if writer.mode.matches(&mode) {
            return;
        }

        set_vertex_attribs(&[2, 4]);
        writer.def_rect_shader.enable();
        writer.mode = mode;
    }

    const MVM_NAME: [u8; 4] = [b'm', b'v', b'm', 0];
    fn set_tf_mat(mat: Matrix3x3) {
        let reader = GRAPHICS.read().unwrap();
        let shader = match reader.get_current_shader() {
            Some(x) => x,
            None => return,
        };

        let tf_mat_address = gl_call!(gl::GetUniformLocation(shader.id(), Self::MVM_NAME.as_ptr() as *const i8));
        assert!(tf_mat_address != -1);

        let mvm = &reader.curr_cam_mat * &mat;

        shader.enable(); // must enable for the next gl_call to not fucking scream and die
        gl_call!(gl::UniformMatrix3fv(tf_mat_address, 1, gl::TRUE, mvm.ptr()))
    }

    fn get_current_shader(&self) -> Option<&Shader> {
        match self.mode {
            Mode::Unset => None,
            Mode::Rect => Some(&self.def_rect_shader),
            Mode::_Custom => None,
        }
    }
}


