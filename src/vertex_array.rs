use glow::*;

pub struct VertexArray {
    id: NativeVertexArray,
}

impl VertexArray {
    pub fn new(gl: &Context) -> Result<Self, String> {
        let id = unsafe { gl.create_vertex_array().map_err(|e| e.to_string())? };
        Ok(Self { id })
    }

    pub fn bind(&self, gl: &Context) {
        unsafe {
            gl.bind_vertex_array(Some(self.id));
        }
    }

    pub fn unbind(&self, gl: &Context) {
        unsafe {
            gl.bind_vertex_array(None);
        }
    }

    pub fn set_attribute_f32(
        &self,
        gl: &Context,
        index: u32,
        size: i32,
        stride: i32,
        offset: i32,
    ) {
        self.bind(gl);
        unsafe {
            gl.vertex_attrib_pointer_f32(index, size, glow::FLOAT, false, stride, offset);
            gl.enable_vertex_attrib_array(index);
        }
    }

    pub fn delete(&self, gl: &Context) {
        unsafe {
            gl.delete_vertex_array(self.id);
        }
    }
}
