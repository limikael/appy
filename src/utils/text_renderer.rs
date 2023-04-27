use crate::{gl, gl::types::*};
use crate::{utils::*,types::*};
//extern crate nalgebra_glm as glm;

/// Render text on screen.
pub struct TextRenderer {
    program: ShaderProgram,
    buf: ArrayBuffer,
    loc_vertex: u32,
    loc_tex_coord: u32,
    loc_col: i32,
    loc_mvp: i32,
    pub window_width: i32,
    pub window_height: i32,
}

impl TextRenderer {
    /// Create a text renderer for a specified window size.
    pub fn new(window_width:i32, window_height:i32) -> Self {
        let program = ShaderProgram::new(vec![
            ShaderSource::VertexShader("
				#version 300 es
				precision mediump float;
				uniform mat4 mvp;
				in vec2 vertex;
				in vec2 tex_coord;
				out vec2 fragment_tex_coord;
				void main() {
					gl_Position=mvp*vec4(vertex,0.0,1.0);
					fragment_tex_coord=tex_coord;
				}
			".to_string()),
            ShaderSource::FragmentShader("
				#version 300 es
				precision mediump float;
				uniform vec4 col;
				uniform sampler2D texture0;
				in vec2 fragment_tex_coord;
				out vec4 fragment_color;
				void main() {
					vec4 tex_data=texture(texture0,fragment_tex_coord);
					fragment_color=vec4(col.r,col.g,col.b,tex_data.r);
				}
			".to_string()),
        ]);

        Self {
            loc_vertex: program.get_attrib_location("vertex"),
            loc_tex_coord: program.get_attrib_location("tex_coord"),
            loc_col: program.get_uniform_location("col"),
            loc_mvp: program.get_uniform_location("mvp"),
            buf: ArrayBuffer::new(4),
            program,
            window_width,
            window_height,
        }
    }

    /// Draw text.
    pub fn draw(&mut self, str: &str, mut x: f32, y: f32, fx: &Font, col: u32) {
        let m = nalgebra_glm::ortho(
            0.0,
            self.window_width as f32,
            self.window_height as f32,
            0.0,
            -1.0,
            1.0,
        );
        let c = nalgebra_glm::vec4(
            ((col & 0xff0000) >> 16) as f32 / 255.0,
            ((col & 0x00ff00) >> 8) as f32 / 255.0,
            (col & 0x0000ff) as f32 / 255.0,
            1.0,
        );

        let mut data=vec![];
        for c in str.chars() {
            let cinfo=fx.character_infos.get(&c).unwrap();
            let txmin=(cinfo.tex_coords.0, cinfo.tex_coords.1);
            let txmax=(cinfo.tex_coords.0+cinfo.tex_size.0, cinfo.tex_coords.1+cinfo.tex_size.1);
            let scmin=(
                x+fx.size*cinfo.left_padding/2.0,
                y+fx.size*(1.0-cinfo.height_over_line)+fx.v_metrics.descent
            );
            let scmax=(scmin.0+fx.size*cinfo.size.0,scmin.1+fx.size*cinfo.size.1);

            data.append(&mut vec![
                scmin.0, scmin.1,  txmin.0, txmin.1,
                scmax.0, scmin.1,  txmax.0, txmin.1,
                scmax.0, scmax.1,  txmax.0, txmax.1,
                scmin.0, scmin.1,  txmin.0, txmin.1,
                scmax.0, scmax.1,  txmax.0, txmax.1,
                scmin.0, scmax.1,  txmin.0, txmax.1,
            ]);

            x+=fx.size*(cinfo.size.0+cinfo.left_padding/2.0+cinfo.right_padding/2.0);
        }

        self.buf.set_data(data);
        self.program.use_program();
        self.buf.bind(self.loc_vertex, 0, 2);
        self.buf.bind(self.loc_tex_coord, 2, 2);
        fx.bind();

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);

            gl::Uniform4fv(self.loc_col, 1, c.as_ptr());
            gl::UniformMatrix4fv(self.loc_mvp, 1, gl::FALSE, m.as_ptr());
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::DrawArrays(gl::TRIANGLES, 0, self.buf.len() as i32);
        }
    }
}
