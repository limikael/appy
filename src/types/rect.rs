//use std::ops::{Add,Neg};

/// Holds information about a rectangle.
#[derive(Clone)]
pub struct Rect {
	pub x: f32,
	pub y: f32,
	pub w: f32,
	pub h: f32
}

impl Rect {
	pub fn abs(&self, x:f32, y:f32, w:f32, h:f32)->Rect {
		Self{
			x: self.x+x,
			y: self.y+y,
			w,
			h,
		}
	}

	pub fn contains(&self, x:f32, y:f32)->bool {
		if x>=self.x && 
				y>=self.y && 
				x <self.x+self.w && 
				y <self.y+self.h {
			return true;					
		}

		false
	}

	pub fn vflip(self: Self)->Rect {
		Self {
			x: self.x,
			y: self.y+self.h,
			w: self.w,
			h: -self.h,
		}
	}

	pub fn hflip(self: Self)->Rect {
		Self {
			x: self.x+self.w,
			y: self.y,
			w: -self.w,
			h: self.h,
		}
	}

	pub fn hvflip(self: Self)->Rect {
		self.vflip().hflip()
	}

	pub fn edge(self: Self, edge: u32, size:f32)->Self {
		match edge {
			0=>Self{x:self.x, w:self.w, y:self.y, h:size},
			1=>Self{x:self.x+self.w, w:-size, y:self.y, h:self.h},
			2=>Self{x:self.x, w:self.w, y:self.y+self.h, h:-size},
			3=>Self{x:self.x, w:size, y:self.y, h:self.h},
			_=>{panic!("unknown edge")}
		}
	}

	/*pub fn empty()->Self {
		Self {x:0 as T, y:0 as T, w:0 as T, h:0 as T}
	}*/
}