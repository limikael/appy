use crate::{gl, gl::types::*};
use crate::{types::*, utils::*};
use rusttype::{gpu_cache::Cache, PositionedGlyph};
extern crate nalgebra_glm as glm;

/// Render text on screen.
pub struct TextRenderer {
    program: ShaderProgram,
    buf: ArrayBuffer,
    tex_id: u32,
    loc_vertex: i32,
    loc_tex_coord: i32,
    loc_col: i32,
    loc_mvp: i32,
    pub window_width: f32,
    pub window_height: f32,
    cache: Cache<'static>,
    used_glyphs: Vec<PositionedGlyph<'static>>,
    pixel_ratio: f32
}

pub struct TextRendererSpec<'a> {
    pub text: &'a str,
    pub x: f32,
    pub y: f32,
    pub font: &'a Font,
    pub size: f32,
    pub col: u32,
//    pub pr: f32,
    pub alpha: f32
}

impl TextRenderer {
    /// Create a text renderer for a specified window size.
    pub fn new(window_width: f32, window_height: f32, pixel_ratio:f32) -> Self {
        let cache: Cache<'static> = Cache::builder()
            .dimensions(0, 0)
            .scale_tolerance(pixel_ratio)
            .position_tolerance(pixel_ratio)
            .build();

        let mut tex_id: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut tex_id);
        }

        let program = ShaderProgram::new(vec![
            ShaderSource::VertexShader(
                "
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
            "
                .to_string(),
            ),
            ShaderSource::FragmentShader(
                "
                #version 300 es
                precision mediump float;
                uniform vec4 col;
                uniform sampler2D texture0;
                in vec2 fragment_tex_coord;
                out vec4 fragment_color;
                void main() {
                    vec4 tex_data=texture(texture0,fragment_tex_coord);
                    fragment_color=vec4(col.r,col.g,col.b,col.a*tex_data.r);
                }
            "
                .to_string(),
            ),
        ]);

        let mut slf = Self {
            loc_vertex: program.get_attrib_location("vertex"),
            loc_tex_coord: program.get_attrib_location("tex_coord"),
            loc_col: program.get_uniform_location("col"),
            loc_mvp: program.get_uniform_location("mvp"),
            buf: ArrayBuffer::new(4),
            program,
            tex_id,
            window_width,
            window_height,
            cache,
            used_glyphs: vec![],
            pixel_ratio
        };

        slf.set_cache_size(1);
        slf
    }

    fn set_cache_size(&mut self, size: u32) {
        println!("font cache size: {:?}x{:?}",size,size);

        self.cache
            .to_builder()
            .dimensions(size, size)
            .rebuild(&mut self.cache);

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.tex_id);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::TexImage2D(
                gl::TEXTURE_2D, // target
                0,              // level
                gl::R8 as i32,
                size as i32, // width
                size as i32, // height
                0,           // border, must be 0
                gl::RED as u32,
                gl::UNSIGNED_BYTE,
                std::ptr::null(),
            );
        }
    }

    fn render_cache(&mut self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.tex_id);
        }

        let mut cache_misses = 0;
        let mut do_build = true;
        while do_build {
            let res = self.cache.cache_queued(|rect, data| {
                //println!("populate font cache: {:?}",data.as_ptr());
                unsafe {
                    gl::ActiveTexture(gl::TEXTURE0);
                    gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
                    gl::TexSubImage2D(
                        gl::TEXTURE_2D,
                        0,
                        rect.min.x as i32,
                        rect.min.y as i32,
                        (rect.width()) as i32,
                        rect.height() as i32,
                        gl::RED,
                        gl::UNSIGNED_BYTE,
                        data.as_ptr() as *const _,
                    );
                }

                cache_misses += 1;
            });

            match res {
                Err(_) => {
                    cache_misses = 0;
                    self.set_cache_size(self.cache.dimensions().0 * 2);
                }
                Ok(_) => {
                    do_build = false;
                }
            };
        }

        if cache_misses > 0 {
            //println!("Font cache misses: {:?}",cache_misses);
        }
    }

    fn vertices_for(&self, glyph: &PositionedGlyph, pr: f32) -> Vec<f32> {
        let rect = self.cache.rect_for(0, glyph).unwrap();
        if rect.is_none() {
            return vec![];
        }

        let (uv, screen) = rect.unwrap();
        vec![
            screen.min.x as f32 / pr,
            screen.min.y as f32 / pr,
            uv.min.x,
            uv.min.y,
            screen.max.x as f32 / pr,
            screen.min.y as f32 / pr,
            uv.max.x,
            uv.min.y,
            screen.max.x as f32 / pr,
            screen.max.y as f32 / pr,
            uv.max.x,
            uv.max.y,
            screen.min.x as f32 / pr,
            screen.min.y as f32 / pr,
            uv.min.x,
            uv.min.y,
            screen.max.x as f32 / pr,
            screen.max.y as f32 / pr,
            uv.max.x,
            uv.max.y,
            screen.min.x as f32 / pr,
            screen.max.y as f32 / pr,
            uv.min.x,
            uv.max.y,
        ]
    }

    /// Draw text.
    pub fn draw(&mut self, spec: &TextRendererSpec) {
        let m = glm::ortho(0.0, self.window_width, self.window_height, 0.0, -1.0, 1.0);
        let c = glm::vec4(
            ((spec.col & 0xff0000) >> 16) as f32 / 255.0,
            ((spec.col & 0x00ff00) >> 8) as f32 / 255.0,
            (spec.col & 0x0000ff) as f32 / 255.0,
            spec.alpha,
        );

        let mut y=spec.y;
        y += spec.font.baseline(spec.size);

        let glyphs = spec.font.create_glyphs(
            spec.text, 
            spec.x * self.pixel_ratio, 
            y * self.pixel_ratio, 
            spec.size * self.pixel_ratio
        );

        for glyph in glyphs.clone() {
            self.used_glyphs.push(glyph.clone());
            self.cache.queue_glyph(0, glyph);
        }

        self.render_cache();
        let mut data: Vec<f32> = vec![];
        for glyph in glyphs {
            data.append(&mut self.vertices_for(&glyph, self.pixel_ratio));
        }

        self.buf.set_data(data);

        //println!("tex id: {}",self.tex_id);

        self.program.use_program();
        self.buf.bind(self.loc_vertex, 0, 2);
        self.buf.bind(self.loc_tex_coord, 2, 2);

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.tex_id);
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

    pub fn begin_frame(&mut self) {
        self.used_glyphs = vec![];
    }

    pub fn end_frame(&mut self) {
        for glyph in self.used_glyphs.clone() {
            self.cache.queue_glyph(0, glyph);
        }

        self.render_cache();
        self.used_glyphs = vec![];
    }
}
