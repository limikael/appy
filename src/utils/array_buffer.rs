/// Abstraction for an OpenGL array buffer.
///
/// This array buffer can hold a number of floating point components.
/// It is set as a flat array, but should be thought of as a two
/// dimensional array. The number of components is the number of 
/// columns in the array. The array buffer can be bound to several
/// attribute locations, with each attribute location using one or
/// more components.
pub struct ArrayBuffer {
    vao: gl::types::GLuint,
    vbo: gl::types::GLuint,
    components: u32,
    len: usize,
}

impl ArrayBuffer {

    /// Create an [`ArrayBuffer`] with a specified number of components.
    pub fn new(components: u32) -> Self {
        let mut vao:gl::types::GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
        }

        let mut vbo: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }

        Self {
            vao,
            vbo,
            components,
            len: 0,
        }
    }

    /// Get the number of items in the array.
    pub fn len(&self) -> usize {
        self.len
    }

    /*pub fn is_empty(&self) -> bool {
        self.len == 0
    }*/

    /// Set data.
    ///
    /// The vertices array is a flat array, but should be thought of
    /// as a two dimensional array. The length of the [`ArrayBuffer`] will
    /// be the lenght of the array divided by the number of components.
    pub fn set_data(&mut self, vertices: Vec<f32>) {
        self.len = vertices.len() / self.components as usize;

        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,                                                       // target
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
                vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::STATIC_DRAW,                               // usage
            );
        }
    }

    /// Bind array buffer to an attribute location.
    ///
    /// The `offs` parameter specifies the first component to bind, and `num`
    /// specifies the number of compontns to bind.
    pub fn bind(&self, attrib_location: u32, offs: usize, num: u32) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::EnableVertexAttribArray(attrib_location); // this is "layout (location = 0)" in vertex shader
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::VertexAttribPointer(
                attrib_location, // index of the generic vertex attribute ("layout (location = 0)")
                num as i32,      // the number of components per generic vertex attribute
                gl::FLOAT,       // data type
                gl::FALSE,       // normalized (int-to-float conversion)
                ((self.components as usize) * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
                (offs * std::mem::size_of::<f32>()) as *const _, // offset of the first component
            );
        }
    }
}
