#[derive(Clone)]
pub struct Rect {
	pub x: i32,
	pub y: i32,
	pub w: i32,
	pub h: i32
}

impl Rect {
	pub fn abs(&self, x:i32, y:i32, w:i32, h:i32)->Rect {
		Self{
			x: self.x+x,
			y: self.y+y,
			w,
			h,
		}
	}

	pub fn contains(&self, x:i32, y:i32)->bool {
		if x>=self.x && 
				y>=self.y && 
				x <self.x+self.w && 
				y <self.y+self.h {
			return true;					
		}

		false
	}
}