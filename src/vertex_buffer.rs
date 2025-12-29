use glow::*;

pub struct VertexBuffer {
    id: NativeBuffer,
}

impl VertexBuffer {
    pub fn new(gl: &Context) -> Result<Self, String> {
        let id = unsafe { gl.create_buffer().map_err(|e| e.to_string())? };
        Ok(Self { id })
    }

    pub fn bind(&self, gl: &Context) {
        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.id));
        }
    }

    pub fn unbind(&self, gl: &Context) {
        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, None);
        }
    }

    pub fn set_data<T>(&self, gl: &Context, data: &[T], usage: u32) {
        self.bind(gl);
        unsafe {
            let bytes = core::slice::from_raw_parts(
                data.as_ptr() as *const u8,
                data.len() * core::mem::size_of::<T>(),
            );
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, bytes, usage);
        }
    }

    pub fn delete(&self, gl: &Context) {
        unsafe {
            gl.delete_buffer(self.id);
        }
    }
}
