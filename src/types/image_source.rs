use crate::gl;
use crate::gl::types::*;
//use std::path::Path;
use image::EncodableLayout;

/// An image to be used with the [img](appy::components::Img) component.
///
/// An image source can only be created from memory, e.g. together with
/// the `include_bytes!` macro. There is currently no way to load an
/// image from a file or other assets.
#[derive(Debug)]
pub struct ImageSource {
    id: GLuint,
    pub width: i32,
    pub height: i32,
}

impl ImageSource {
    pub fn from_memory(buffer: &[u8]) -> Self {
        let mut im = Self::gen();
        im.process_image(image::load_from_memory(buffer).expect("Unable to load image"));
        im
    }

    // This only works for desktop.
    // TODO: Find a way to load android assets (and other assets).
    /*pub fn load(path:&str)->Self {
        let mut im=Self::gen();
        im.process_image(image::open(path).expect("Unable to load image"));
        im
    }*/

    fn gen() -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
        }
        Self {
            id,
            width: -1,
            height: -1,
        }
    }

    fn process_image(&mut self, dyn_image: image::DynamicImage) {
        self.bind();

        let img = dyn_image.into_rgba8();
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img.as_bytes().as_ptr() as *const _,
            );
        }

        self.width = img.width() as i32;
        self.height = img.height() as i32;
    }

    pub fn bind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, self.id) }
    }
}

impl Drop for ImageSource {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, [self.id].as_ptr());
        }
    }
}
