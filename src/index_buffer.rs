use glow::*;

pub struct IndexBuffer {
    id: NativeBuffer,
    count: i32,
}

impl IndexBuffer {
    pub fn new(gl: &Context, indices: &[u32], usage: u32) -> Result<Self, String> {
        let id = unsafe { gl.create_buffer().map_err(|e| e.to_string())? };
        let ibo = Self {
            id,
            count: indices.len() as i32,
        };
        ibo.set_data(gl, indices, usage);
        Ok(ibo)
    }

    pub fn bind(&self, gl: &Context) {
        unsafe {
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.id));
        }
    }

    pub fn unbind(&self, gl: &Context) {
        unsafe {
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
        }
    }

    pub fn set_data(&self, gl: &Context, indices: &[u32], usage: u32) {
        self.bind(gl);
        unsafe {
            let bytes = core::slice::from_raw_parts(
                indices.as_ptr() as *const u8,
                indices.len() * core::mem::size_of::<u32>(),
            );
            gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, bytes, usage);
        }
    }

    pub fn delete(&self, gl: &Context) {
        unsafe {
            gl.delete_buffer(self.id);
        }
    }

    pub fn count(&self) -> i32 {
        self.count
    }
}
