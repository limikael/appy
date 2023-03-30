use rusttype::{Font, Scale, point, Point, PositionedGlyph};
use rusttype::gpu_cache::{Cache};
use gl::types::{GLint, GLuint};
use crate::{*};
extern crate nalgebra_glm as glm;

pub struct TextRenderer {
	program: ShaderProgram,
	buf: ArrayBuffer,
	tex_id: u32,
	loc_vertex: u32,
	loc_tex_coord: u32,
	loc_col: i32,
	loc_mvp: i32,
	pub window_width: i32,
	pub window_height: i32,
	font: Font<'static>,
	cache: Cache<'static>
}

impl TextRenderer {
	//const CACHE_SIZE:u32=512;
	const CACHE_SIZE:u32=1024;

	pub fn new()->Self {
		let font_data=include_bytes!("./assets/Roboto-Regular.ttf");
		let font=Font::try_from_bytes(font_data as &[u8]).unwrap();

		let cache:Cache<'static>=Cache::builder()
			.dimensions(Self::CACHE_SIZE as u32,Self::CACHE_SIZE as u32)
			.build();

		let mut tex_id: GLuint=0;
		unsafe { gl::GenTextures(1, &mut tex_id); }
		unsafe { gl::BindTexture(gl::TEXTURE_2D, tex_id); }

		unsafe {
			gl::ActiveTexture(gl::TEXTURE0+0);

			gl::TexImage2D(
				gl::TEXTURE_2D,				// target
				0,							// level
				gl::ALPHA as i32,				// internal format
				Self::CACHE_SIZE as i32,						// width
				Self::CACHE_SIZE as i32,						// height
				0,							// border, must be 0
				gl::ALPHA,					// format
				gl::UNSIGNED_BYTE,
				0 as *const _
			);
		}

		let program=ShaderProgram::new(vec![
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
					fragment_color=vec4(col.r,col.g,col.b,tex_data.a);
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
			tex_id,
			window_width: 100,
			window_height: 100,
			font,
			cache
		}
	}

	fn get_glyph_advance(&self, c:char, s:Scale)->(f32,f32) {
		let g=self.font.glyph(c).scaled(s);
		let h=g.h_metrics().advance_width;
		let v_metrics=self.font.v_metrics(s);
		let v = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;
		(h,v)
	}

	pub fn get_str_width(&self, str:&str, size:f32)->f32 {
		let mut w:f32=0.0;
		let s=Scale::uniform(size);

		for c in str.chars() {
			let (adv_x,_adv_y)=self.get_glyph_advance(c,s);
			w+=adv_x;
		}

		w
	}

	fn create_glyphs<'a>(&self, str:&str, mut p:Point<f32>, s:Scale)->Vec<PositionedGlyph<'a>> {
		let mut v=Vec::new();

		for c in str.chars() {
			let base_glyph=self.font.glyph(c);
			v.push(base_glyph.scaled(s).positioned(p));

			let (adv_x,_adv_y)=self.get_glyph_advance(c,s);
			p=point(p.x+adv_x,p.y);
		}

		v
	}

	fn render_cache(&mut self) {
		self.cache.cache_queued(|rect, data| {
			//log_debug!("copy data: {:?}",data.as_ptr());
			unsafe {
				gl::ActiveTexture(gl::TEXTURE0+0);
				gl::PixelStorei(gl::UNPACK_ALIGNMENT,1);
				gl::TexSubImage2D(
					gl::TEXTURE_2D,
					0,
					rect.min.x as i32,
					rect.min.y as i32,
					(rect.width()) as i32,
					rect.height() as i32,
					gl::ALPHA,
					gl::UNSIGNED_BYTE,
					data.as_ptr() as *const _
				);
			}
		}).unwrap();
	}

	fn vertices_for(&self, glyph:&PositionedGlyph)->Vec<f32> {
		let rect=self.cache.rect_for(0,&glyph).unwrap();
		if !rect.is_some() {
			return vec![]
		}

		let (uv,screen)=rect.unwrap();

		vec![
			screen.min.x as f32,screen.min.y as f32, uv.min.x,uv.min.y,
			screen.max.x as f32,screen.min.y as f32, uv.max.x,uv.min.y,
			screen.max.x as f32,screen.max.y as f32, uv.max.x,uv.max.y,

			screen.min.x as f32,screen.min.y as f32, uv.min.x,uv.min.y,
			screen.max.x as f32,screen.max.y as f32, uv.max.x,uv.max.y,
			screen.min.x as f32,screen.max.y as f32, uv.min.x,uv.max.y,
		]
	}

	pub fn draw(&mut self, str:&str, x:f32, y:f32, size:f32, col:u32) {
		let m=glm::ortho(0.0,self.window_width as f32,self.window_height as f32,0.0,-1.0,1.0);
		let c=glm::vec4(
			((col&0xff0000)>>16) as f32/255.0,
			((col&0x00ff00)>>8) as f32/255.0,
			((col&0x0000ff)>>0) as f32/255.0,
			1.0
		);

		let s=Scale::uniform(size);
		let v_metrics=self.font.v_metrics(s);
		let p=point(x,y+size+v_metrics.descent);

		let glyphs=self.create_glyphs(str,p,s);
		for glyph in glyphs.clone() {
			self.cache.queue_glyph(0,glyph);
		}

		self.render_cache();
		let mut data:Vec<f32>=vec![];
		for glyph in glyphs {
			let mut v=self.vertices_for(&glyph);
			data.append(&mut v);
		}

		self.buf.set_data(data);

		//println!("tex id: {}",self.tex_id);

		self.program.use_program();
		self.buf.bind(self.loc_vertex,0,2);
		self.buf.bind(self.loc_tex_coord,2,2);

		unsafe {
			gl::ActiveTexture(gl::TEXTURE0+0);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
			gl::BindTexture(gl::TEXTURE_2D, self.tex_id);
			gl::Uniform4fv(self.loc_col,1,c.as_ptr());
			gl::UniformMatrix4fv(self.loc_mvp,1,gl::FALSE,m.as_ptr());
			gl::Enable(gl::BLEND);
			gl::BlendFunc(gl::SRC_ALPHA,gl::ONE_MINUS_SRC_ALPHA);
			gl::DrawArrays(gl::TRIANGLES,0,self.buf.len() as i32);
		}
	}
}