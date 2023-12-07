use crate::log_warn;

use super::{shader::Shader, uniforms::Uniform, gl_call};

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    shader: Shader,
    uniforms: Vec<(gl::types::GLint, Uniform)>
}

impl Material {
    pub(super) const fn invalid() -> Self {
        Self { shader: Shader::invalid(), uniforms: Vec::new() }
    }
    
    pub fn new(shader: &Shader, uniforms: &[(&[u8], Uniform)]) -> Self {
        let uniforms = uniforms.iter().filter_map(|(k, v)| {
            assert!(k.len() != 0 && k[k.len() - 1] == b'\0', "All uniform names must be zero terminated u8 slices");
            let id = gl_call!(gl::GetUniformLocation(shader.id(), k.as_ptr() as *const i8));
            if id == -1 {
                log_warn!("Uniform '{}' was not present for the provided shader. Value will be skipped", std::str::from_utf8(k).unwrap());
                return None;
            }
            return Some((id, v.clone()));
        }).collect::<Vec<_>>();

        return Self { shader: shader.clone(), uniforms };
    }

    pub fn set_uniform_by_name(&mut self, name: &[u8], value: Uniform) {
        let id = match self.get_uniform_address(name) {
            Some(x) => x,
            None => {
                log_warn!("Uniform '{}' was not present for the provided shader. Value will be skipped", std::str::from_utf8(name).unwrap());
                return;
            },
        };

        self.set_uniform(id, value);
    }

    pub fn set_uniform(&mut self, address: i32, value: Uniform) {
        assert!(address >= 0, "Address must be postive.");

        match self.uniforms.iter().position(|x| x.0 == address) {
            Some(i) => {
                self.uniforms[i].1 = value;
            },
            None => {
                self.uniforms.push((address, value));
            },
        };
    }

    pub fn get_uniform_address(&self, name: &[u8]) -> Option<i32> {
        assert!(name.len() != 0 && name[name.len() - 1] == b'\0', "All uniform names must be zero terminated u8 slices");
        let id = gl_call!(gl::GetUniformLocation(self.shader.id(), name.as_ptr() as *const i8));

        if id == -1 {
            return None;
        } else {
            return Some(id);
        }
    }

    pub(super) fn enable(&self) {
        self.shader.enable();

        for (l, u) in &self.uniforms {
            match u {
                Uniform::Float(x) => gl_call!(gl::Uniform1f(*l, *x)),
                Uniform::Float2(x, y) => gl_call!(gl::Uniform2f(*l, *x, *y)),
                Uniform::Float3(x, y, z) => gl_call!(gl::Uniform3f(*l, *x, *y, *z)),
                Uniform::Float4(x, y, z, w) => gl_call!(gl::Uniform4f(*l, *x, *y, *z ,*w)),
                Uniform::Int(x) => gl_call!(gl::Uniform1i(*l, *x)),
                Uniform::Uint(x) => gl_call!(gl::Uniform1ui(*l, *x)),
                //Uniform::Texture(x) => gl_call!(gl::Uniform1ui(*l, x.core().id())),
            }
        }
    }

    pub fn shader(&self) -> &Shader {
        &self.shader
    }
}