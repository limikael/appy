extern crate nalgebra_glm as glm;

use crate::gl;
use crate::gl::types::*;
use crate::{types::*, utils::*};

pub struct ImageRenderer {
    program: ShaderProgram,
    buf: ArrayBuffer,
    window_width: f32,
    window_height: f32,
    loc_vertex: i32,
    loc_tex_coord: i32,
    loc_mvp: i32,
    loc_pos: i32,
    loc_size: i32,
}

impl ImageRenderer {
    pub fn new(window_width: f32, window_height: f32) -> Self {
        let program = ShaderProgram::new(vec![
            ShaderSource::VertexShader(
                "
				#version 300 es
				precision mediump float;
				uniform mat4 mvp;
				uniform vec2 pos,size;
				in vec2 vertex;
				in vec2 tex_coord;
				out vec2 fragment_tex_coord;
				void main() {
					gl_Position=mvp*vec4(pos+vertex*size,0.0,1.0);
					fragment_tex_coord=tex_coord;
				}
			"
                .to_string(),
            ),
            ShaderSource::FragmentShader(
                "
				#version 300 es
				precision mediump float;
				uniform sampler2D texture0;
				in vec2 fragment_tex_coord;
				out vec4 fragment_color;
				void main() {
					vec4 tex_data=texture(texture0,fragment_tex_coord);
					fragment_color=vec4(tex_data.r,tex_data.g,tex_data.b,tex_data.a);
				}
			"
                .to_string(),
            ),
        ]);

        let mut buf = ArrayBuffer::new(4);
        buf.set_data(vec![
            0.0, 0.0, 0.0, 0.0, 
            1.0, 0.0, 1.0, 0.0, 
            1.0, 1.0, 1.0, 1.0, 
            0.0, 0.0, 0.0, 0.0, 
            1.0, 1.0, 1.0, 1.0, 
            0.0, 1.0, 0.0, 1.0,
        ]);

        Self {
            loc_vertex: program.get_attrib_location("vertex"),
            loc_tex_coord: program.get_attrib_location("tex_coord"),
            loc_mvp: program.get_uniform_location("mvp"),
            loc_pos: program.get_uniform_location("pos"),
            loc_size: program.get_uniform_location("size"),
            program,
            window_width,
            window_height,
            buf,
        }
    }

    pub fn set_size(&mut self, window_width: f32, window_height: f32) {
        self.window_width = window_width;
        self.window_height = window_height;
    }

    pub fn draw(&self, rect: &Rect, image: &ImageSource) {
        let m = glm::ortho(
            0.0,
            self.window_width as f32,
            self.window_height as f32,
            0.0,
            -1.0,
            1.0,
        );

        image.bind();

        self.program.use_program();
        self.buf.bind(self.loc_vertex, 0, 2);
        self.buf.bind(self.loc_tex_coord, 2, 2);

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);

            gl::Uniform2f(self.loc_pos, rect.x as f32, rect.y as f32);
            gl::Uniform2f(self.loc_size, rect.w as f32, rect.h as f32);
            gl::UniformMatrix4fv(self.loc_mvp, 1, gl::FALSE, m.as_ptr());
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::DrawArrays(gl::TRIANGLES, 0, self.buf.len() as i32);
        }
    }
}
