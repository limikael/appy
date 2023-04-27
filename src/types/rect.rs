use std::ops::{Add,Neg};

/// Holds information about a rectangle.
#[derive(Clone)]
pub struct Rect<T> {
	pub x: T,
	pub y: T,
	pub w: T,
	pub h: T
}

impl<T: PartialOrd+Copy+Add<Output=T>+Neg<Output=T>> Rect<T> {
	pub fn abs(&self, x:T, y:T, w:T, h:T)->Rect<T> {
		Self{
			x: self.x+x,
			y: self.y+y,
			w,
			h,
		}
	}

	pub fn contains(&self, x:T, y:T)->bool {
		if x>=self.x && 
				y>=self.y && 
				x <self.x+self.w && 
				y <self.y+self.h {
			return true;					
		}

		false
	}

	pub fn vflip(self: Self)->Rect<T> {
		Self {
			x: self.x,
			y: self.y+self.h,
			w: self.w,
			h: -self.h,
		}
	}

	pub fn hflip(self: Self)->Rect<T> {
		Self {
			x: self.x+self.w,
			y: self.y,
			w: -self.w,
			h: self.h,
		}
	}

	pub fn hvflip(self: Self)->Rect<T> {
		self.vflip().hflip()
	}

	pub fn edge(self: Self, edge: u32, size:T)->Self {
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