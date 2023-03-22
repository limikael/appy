extern crate nalgebra_glm as glm;
use crate::{*};

pub struct RectRenderer {
	program: ShaderProgram,
	buf: ArrayBuffer,
	pub window_width: i32,
	pub window_height: i32
}

impl RectRenderer {
	pub fn new()->Self {
		let mut program=ShaderProgram::new();

		program.add_vertex_shader("
			#version 330 core
			in vec3 Position;
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
			uniform vec4 col;

			void main() {
			    Color = col*vec4(1.0f, 1.0f, 1.0f, 1.0f);
			}
		");

		program.link();

		let mut buf=ArrayBuffer::new(2);
		buf.set_data(vec![
			0.0, 0.0,
			1.0, 0.0,
			1.0, 1.0,
			0.0, 1.0,
		]);

		RectRenderer{
			program: program,
			buf: buf,
			window_width: 100,
			window_height: 100
		}
	}

	pub fn draw(&self, rect:&Rect, col:u32) {
		let m=glm::ortho(0.0,self.window_width as f32,self.window_height as f32,0.0,-1.0,1.0);
		let c=glm::vec4(
			((col&0xff0000)>>16) as f32/255.0,
			((col&0x00ff00)>>8) as f32/255.0,
			((col&0x0000ff)>>0) as f32/255.0,
			1.0
		);

		self.program.use_program();
		self.buf.bind(self.program.get_attrib_location("Position"),0,2);

		unsafe { 
			gl::Uniform1f(self.program.get_uniform_location("left"),rect.x as f32); 
			gl::Uniform1f(self.program.get_uniform_location("top"),rect.y as f32); 
			gl::Uniform1f(self.program.get_uniform_location("width"),rect.w as f32); 
			gl::Uniform1f(self.program.get_uniform_location("height"),rect.h as f32); 
			gl::Uniform4fv(self.program.get_uniform_location("col"),1,c.as_ptr());

			gl::UniformMatrix4fv(self.program.get_uniform_location("MVP"),1,gl::FALSE,m.as_ptr())
		}

		unsafe {
			gl::Disable(gl::BLEND);
			gl::DrawArrays(gl::QUADS,0,4);
    	}

		//self.buf.draw();
	}
}
