use glow::{Context, HasContext, NativeProgram, NativeUniformLocation};

pub struct Program {
    id: NativeProgram,
}

impl Program {
    pub fn new(
        gl: &Context,
        vertex_shader_source: &str,
        fragment_shader_source: &str,
    ) -> Result<Self, String> {
        let program = unsafe { gl.create_program().map_err(|e| e.to_string())? };

        let shader_sources = [
            (glow::VERTEX_SHADER, vertex_shader_source),
            (glow::FRAGMENT_SHADER, fragment_shader_source),
        ];

        let mut shaders = Vec::with_capacity(shader_sources.len());

        for (shader_type, shader_source) in shader_sources.iter() {
            let shader = unsafe { gl.create_shader(*shader_type).map_err(|e| e.to_string())? };
            unsafe {
                gl.shader_source(shader, shader_source);
                gl.compile_shader(shader);
                if !gl.get_shader_compile_status(shader) {
                    return Err(gl.get_shader_info_log(shader));
                }
                gl.attach_shader(program, shader);
            }
            shaders.push(shader);
        }

        unsafe {
            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                return Err(gl.get_program_info_log(program));
            }

            for shader in shaders {
                gl.detach_shader(program, shader);
                gl.delete_shader(shader);
            }
        }

        Ok(Self { id: program })
    }

    pub fn bind(&self, gl: &Context) {
        unsafe {
            gl.use_program(Some(self.id));
        }
    }

    pub fn delete(&self, gl: &Context) {
        unsafe {
            gl.delete_program(self.id);
        }
    }

    pub fn get_uniform_location(&self, gl: &Context, name: &str) -> Option<NativeUniformLocation> {
        unsafe { gl.get_uniform_location(self.id, name) }
    }

    pub fn set_uniform_4f(&self, gl: &Context, location: &NativeUniformLocation, x: f32, y: f32, z: f32, w: f32) {
        unsafe {
            gl.uniform_4_f32(Some(location), x, y, z, w);
        }
    }

    pub fn set_uniform_mat4(&self, gl: &Context, location: &NativeUniformLocation, mat: &crate::math::Mat4) {
        unsafe {
            gl.uniform_matrix_4_f32_slice(Some(location), false, &mat.data);
        }
    }
}
