use std::ffi::CString;

pub struct ShaderProgram {
	pub program_id: gl::types::GLuint,
	is_linked: bool
}

impl ShaderProgram {
	pub fn new() -> Self {
		Self{
			program_id: unsafe { gl::CreateProgram() },
			is_linked: false
		}
	}

	pub fn link(&mut self) {
		if !self.is_linked {
			unsafe {
				gl::LinkProgram(self.program_id);
			}

			self.is_linked=true;
		}
	}

	fn ensure_linked(&self) {
		if !self.is_linked {
			panic!("not linked");
		}
	}

	pub fn use_program(&self) {
		self.ensure_linked();
		unsafe { gl::UseProgram(self.program_id); }
	}

	pub fn add_shader(&self, source: &str, kind: gl::types::GLuint) {
		let str_source=CString::new(source).unwrap();
		let shader_id=unsafe { gl::CreateShader(kind) };

		unsafe {
			gl::ShaderSource(shader_id, 1, &str_source.as_ptr(), std::ptr::null());
			gl::CompileShader(shader_id);
		}

		let mut success: gl::types::GLint = 1;
		unsafe {
			gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);
		}

		if success==0 {
			panic!("Unable to compile shader")
		}

		unsafe {
			gl::AttachShader(self.program_id, shader_id);
		}
	}

	pub fn add_vertex_shader(&self, source: &str) {
		self.add_shader(source,gl::VERTEX_SHADER);
	}

	pub fn add_fragment_shader(&self, source: &str) {
		self.add_shader(source,gl::FRAGMENT_SHADER);
	}

	pub fn get_uniform_location(&self, name: &str)->i32 {
		self.ensure_linked();

		let s=CString::new(name).unwrap();
		let loc:i32;

		unsafe {
			loc=gl::GetUniformLocation(self.program_id,s.as_ptr());
		}

		if loc < 0 {
			panic!("Can't find uniform")
		}

		return loc;
	}

	pub fn get_attrib_location(&self, name: &str)->u32 {
		self.ensure_linked();

		let s=CString::new(name).unwrap();
		let loc:i32;

		unsafe {
			loc=gl::GetAttribLocation(self.program_id,s.as_ptr());
		}

		if loc < 0 {
			panic!("Can't find uniform")
		}

		return loc as u32;
	}
}
