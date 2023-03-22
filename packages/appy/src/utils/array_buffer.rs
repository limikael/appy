pub struct ArrayBuffer {
//	_vertices: Vec<f32>,
	vbo: gl::types::GLuint,
	components: u32,
	len: usize
}

impl ArrayBuffer {
	pub fn new(components:u32)->Self {
		// set up vertex buffer
		let mut vbo: gl::types::GLuint = 0;
		unsafe {
			gl::GenBuffers(1, &mut vbo);
		}

		/*unsafe {
			gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
			gl::BufferData(
				gl::ARRAY_BUFFER,                                                       // target
				(vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
				vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
				gl::STATIC_DRAW,                               // usage
			);
		}*/

		return Self {
//			_vertices: vertices,
			vbo,
			components,
			len: 0
		};
	}

	pub fn len(&self)->usize {
		self.len
	}

	pub fn set_data(&mut self, vertices:Vec<f32>) {
		self.len=vertices.len()/self.components as usize;

		unsafe {
			gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
			gl::BufferData(
				gl::ARRAY_BUFFER,                                                       // target
				(vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
				vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
				gl::STATIC_DRAW,                               // usage
			);
		}
	}

	pub fn bind(&self, attrib_location:u32, offs:usize, num:u32) {
		unsafe {
			gl::EnableVertexAttribArray(attrib_location); // this is "layout (location = 0)" in vertex shader
			gl::BindBuffer(gl::ARRAY_BUFFER,self.vbo);
			gl::VertexAttribPointer(
				attrib_location,         // index of the generic vertex attribute ("layout (location = 0)")
				num as i32,         // the number of components per generic vertex attribute
				gl::FLOAT, // data type
				gl::FALSE, // normalized (int-to-float conversion)
				((self.components as usize) * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
				(offs*std::mem::size_of::<f32>()) as *const _,                                     // offset of the first component
			);
		}
	}
}
