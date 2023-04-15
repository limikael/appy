use super::array_buffer::ArrayBuffer;
use super::rect::Rect;
use super::shader_program::{ShaderProgram, ShaderSource};

extern crate nalgebra_glm as glm;

/// Render rectangles.
pub struct RectRenderer {
    program: ShaderProgram,
    buf: ArrayBuffer,
    pub window_width: i32,
    pub window_height: i32,
}

impl RectRenderer {
    /// Create a RectRenderer
    pub fn new(window_width: i32, window_height: i32) -> Self {
        let program = ShaderProgram::new(vec![
            ShaderSource::VertexShader(
                "
				#version 300 es
				precision mediump float;
				in vec3 Position;
				uniform float left, top, width, height;
				uniform mat4 MVP;

				void main() {
					vec3 scale=vec3(width,height,1.0);
					vec3 pos=vec3(left,top,1.0);
					gl_Position=MVP*vec4(pos+scale*Position, 1.0);
				}
			"
                .to_string(),
            ),
            ShaderSource::FragmentShader(
                "
				#version 300 es
				precision mediump float;
				out vec4 Color;
				uniform vec4 col;

				void main() {
					Color = col*vec4(1.0f, 1.0f, 1.0f, 1.0f);
				}
			"
                .to_string(),
            ),
        ]);

        let mut buf = ArrayBuffer::new(2);
        buf.set_data(vec![1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0]);

        RectRenderer {
            program,
            buf,
            window_width,
            window_height,
        }
    }

    /// Draw a rect, specified by orientation and size.
    pub fn draw(&self, rect: &Rect, col: u32) {
        let m = glm::ortho(
            0.0,
            self.window_width as f32,
            self.window_height as f32,
            0.0,
            -1.0,
            1.0,
        );
        let c = glm::vec4(
            ((col & 0xff0000) >> 16) as f32 / 255.0,
            ((col & 0x00ff00) >> 8) as f32 / 255.0,
            (col & 0x0000ff) as f32 / 255.0,
            1.0,
        );

        self.program.use_program();
        self.buf
            .bind(self.program.get_attrib_location("Position"), 0, 2);

        unsafe {
            gl::Uniform1f(self.program.get_uniform_location("left"), rect.x as f32);
            gl::Uniform1f(self.program.get_uniform_location("top"), rect.y as f32);
            gl::Uniform1f(self.program.get_uniform_location("width"), rect.w as f32);
            gl::Uniform1f(self.program.get_uniform_location("height"), rect.h as f32);
            gl::Uniform4fv(self.program.get_uniform_location("col"), 1, c.as_ptr());

            gl::UniformMatrix4fv(
                self.program.get_uniform_location("MVP"),
                1,
                gl::FALSE,
                m.as_ptr(),
            )
        }

        unsafe {
            gl::Disable(gl::BLEND);
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, self.buf.len() as i32);
        }
    }
}
