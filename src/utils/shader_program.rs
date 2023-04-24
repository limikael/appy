use std::ffi::CString;
use crate::gl::types::{*};
use crate::gl;

fn compile_shader(source: String, kind: GLuint)->GLuint {
	let str_source=CString::new(&*source).unwrap();
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
		let mut len:gl::types::GLint = 0;
		unsafe { gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut len); }
		//log_debug!("l={}",len);

		let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 10);
		buffer.extend([b' '].iter().cycle().take(len as usize));
		let error: CString = unsafe { CString::from_vec_unchecked(buffer) };

		let mut len_out:gl::types::GLint = 0;

		unsafe {
			gl::GetShaderInfoLog(
				shader_id,
				len+5,
				&mut len_out,
				error.as_ptr() as *mut gl::types::GLchar
			);
		}

		let s=error.to_string_lossy();
		panic!("Unable to compile shader: {:?}",s);
	}

	shader_id
}

/// Abstraction for an OpenGL shader program.
pub struct ShaderProgram {
	program_id: GLuint,
}

/// The source of a vertex or fragment shader.
pub enum ShaderSource {
	VertexShader(String),
	FragmentShader(String)
}

impl ShaderProgram {
	/// Create a shader program containing a number of
	/// shader sources.
	pub fn new(sources: Vec<ShaderSource>) -> Self {
		let program_id=unsafe {gl::CreateProgram()};

		for source in sources {
			let shader_id=match source {
				ShaderSource::VertexShader(source_text)=>{
					compile_shader(source_text,gl::VERTEX_SHADER)
				},

				ShaderSource::FragmentShader(source_text)=>{
					compile_shader(source_text,gl::FRAGMENT_SHADER)
				},
			};

			unsafe {gl::AttachShader(program_id, shader_id)};
		}

		unsafe {gl::LinkProgram(program_id)};

		Self{
			program_id
		}
	}

	/// Use this program.
	pub fn use_program(&self) {
		unsafe { gl::UseProgram(self.program_id); }
	}

	/// The the location of a uniform.
	pub fn get_uniform_location(&self, name: &str)->i32 {
		//self.ensure_linked();

		let s=CString::new(name).unwrap();
		let loc:i32;

		unsafe {
			loc=gl::GetUniformLocation(self.program_id,s.as_ptr());
		}

		if loc < 0 {
			panic!("Can't find uniform: {}",name)
		}

		loc
	}

	/// The the location of an attrib.
	pub fn get_attrib_location(&self, name: &str)->u32 {
		//self.ensure_linked();

		let s=CString::new(name).unwrap();
		let loc:i32;

		unsafe {
			loc=gl::GetAttribLocation(self.program_id,s.as_ptr());
		}

		if loc < 0 {
			panic!("Can't find attrib: {}",name)
		}

		loc as u32
	}
}
