use std::rc::Rc;

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
