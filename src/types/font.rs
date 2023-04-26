use std::collections::HashMap;
use crate::{gl, gl::types::*};
use crate::{utils::*, hooks::*};
use rusttype::{Scale, VMetrics};

/// Represents a font face without specified size.
///
/// Creating a `FontFace` is the first step when drawing text.
/// The second step is to create a font with a given size,
/// using [`Font`](Font).
///
/// To obtain a `FontFace`, use the [`use_font_face`](use_font_face) hook.
pub struct FontFace<'a> {
    pub rusttype_font: rusttype::Font<'a>
}

impl<'a> FontFace<'a> {
    pub fn from_data(ttf_data:&'a [u8])->Self {
        let f=rusttype::Font::try_from_bytes(ttf_data).unwrap();

        Self {
            rusttype_font: f,
        }
    }
}

/// Represents a font with a specified size, rendered to a texture.
///
/// To obtain a `Font`, use the [`use_font`](use_font) hook.
pub struct Font {
	id: GLuint,
    /*width: i32,
    height: i32,*/
    pub character_infos: HashMap<char,CharacterInfos>,
    pub size:f32,
    pub v_metrics: VMetrics
}

impl Font {
    pub fn new(font_face: &FontFace, size: f32)->Self {
        let r=[0x20u32..0xff];
        let chars=r.iter().cloned().flatten().map(|c|std::char::from_u32(c).unwrap()); //.collect();
        let (data,w,h,infos)=build_font_image(&font_face.rusttype_font,chars,size as u32).unwrap();

        let mut id: GLuint = 0;
        unsafe { 
            gl::GenTextures(1, &mut id); 
            gl::BindTexture(gl::TEXTURE_2D, id);
        }

        unsafe {
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::R8 as i32,
                w as i32, 
                h as i32,
                0,
                gl::RED as u32,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const _,
            );
        }

        let v_metrics=font_face.rusttype_font.v_metrics(Scale::uniform(size));

        Self {
            id,
            /*width: w as i32,
            height: h as i32,*/
            character_infos: infos,
            size,
            v_metrics
        }
    }

    pub fn bind(&self) {
        unsafe {
        	gl::BindTexture(gl::TEXTURE_2D, self.id)
        }
    }

    pub fn get_str_width(&self, s: &str)->f32 {
        let mut x:f32=0.0;

        for c in s.chars() {
            let cinfo=self.character_infos.get(&c).unwrap();
            x+=self.size*(cinfo.size.0+cinfo.left_padding/2.0+cinfo.right_padding/2.0);
        }

        x
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, [self.id].as_ptr());
        }
    }
}
