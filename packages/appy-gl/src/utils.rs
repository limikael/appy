extern crate nalgebra_glm as glm;
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

	pub fn use_program(&self) {
		if !self.is_linked {
			unsafe {
				gl::LinkProgram(self.program_id);
			}
		}

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
}

pub struct ArrayBuffer {
	vertices: Vec<f32>,
//	vbo: gl::types::GLuint,
	vao: gl::types::GLuint
}

impl ArrayBuffer {
	pub fn new(vertices:Vec<f32>) -> Self {
		// set up vertex buffer
		let mut vbo: gl::types::GLuint = 0;
		unsafe {
			gl::GenBuffers(1, &mut vbo);
		}

		unsafe {
			gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
			gl::BufferData(
				gl::ARRAY_BUFFER,                                                       // target
				(vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
				vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
				gl::STATIC_DRAW,                               // usage
			);
			gl::BindBuffer(gl::ARRAY_BUFFER, 0);
		}

		// set up vertex array object
		let mut vao: gl::types::GLuint = 0;
		unsafe {
			gl::GenVertexArrays(1, &mut vao);
		}

		unsafe {
			gl::BindVertexArray(vao);
			gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
			gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
			gl::VertexAttribPointer(
				0,         // index of the generic vertex attribute ("layout (location = 0)")
				3,         // the number of components per generic vertex attribute
				gl::FLOAT, // data type
				gl::FALSE, // normalized (int-to-float conversion)
				(3 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
				std::ptr::null(),                                     // offset of the first component
			);
			gl::BindBuffer(gl::ARRAY_BUFFER, 0);
			gl::BindVertexArray(0);
		}

		return Self {
			vertices: vertices,
			//vbo: vbo,
			vao: vao
		};
	}

	pub fn draw(&self) {
		unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(
                gl::QUADS, // mode
                0,             // starting index in the enabled arrays
                (self.vertices.len()/3) as i32,             // number of indices to be rendered
            );
        }
	}
}

pub struct RectRenderer {
	program: ShaderProgram,
	buf: ArrayBuffer,
	pub window_width: u32,
	pub window_height: u32
}

impl RectRenderer {
	pub fn new()->Self {
		let program=ShaderProgram::new();

		program.add_vertex_shader("
			#version 330 core
			layout (location = 0) in vec3 Position;
			uniform float left, top, width, height;
			uniform mat4 MVP;

			void main() {
				vec3 scale=vec3(width,height,1.0);
				vec3 pos=vec3(left,top,1.0);
	    		gl_Position=MVP*vec4(pos+scale*Position, 1.0);
			}
		");

		program.add_fragment_shader("
			#version 330 core
			out vec4 Color;
			void main() {
			    Color = vec4(1.0f, 0.5f, 0.2f, 1.0f);
			}
		");

		let buf=ArrayBuffer::new(vec![
			0.0, 0.0, 0.0,
			1.0, 0.0, 0.0,
			1.0, 1.0, 0.0,
			0.0, 1.0, 0.0
		]);

		RectRenderer{
			program: program,
			buf: buf,
			window_width: 100,
			window_height: 100
		}
	}

	pub fn draw(&self, left:i32, top:i32, width:i32, height:i32) {
		let m=glm::ortho(0.0,self.window_width as f32,self.window_height as f32,0.0,-1.0,1.0);

		self.program.use_program();

		unsafe { 
			gl::Uniform1f(self.program.get_uniform_location("left"),left as f32); 
			gl::Uniform1f(self.program.get_uniform_location("top"),top as f32); 
			gl::Uniform1f(self.program.get_uniform_location("width"),width as f32); 
			gl::Uniform1f(self.program.get_uniform_location("height"),height as f32); 

			gl::UniformMatrix4fv(self.program.get_uniform_location("MVP"),1,gl::FALSE,m.as_ptr())
		}

		self.buf.draw();
	}
}