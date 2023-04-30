//use crate::types::{Align,VAlign};

#[derive(Debug)]
pub struct FlowConf {
	pub width: f32,
	pub height: f32,
	pub gap: f32,
	pub vgap: f32,
	/*pub align: Align,
	pub valign: VAlign*/
}

#[derive(Debug)]
struct FlowItem<T> {
	item: T,
	width: f32,
	height: f32,
	x: f32
}

#[derive(Debug)]
struct FlowLine<T> {
	items: Vec<FlowItem<T>>,
	width: f32,
	height: f32,
}

impl<T> FlowLine<T> {
	pub fn new()->Self {
		Self {
			items: vec![],
			width: 0.0,
			height: 0.0
		}
	}

	pub fn can_fit(&self, conf:&FlowConf, width: f32)->bool {
		if self.items.len()==0 {
			return true;
		}

		self.width+conf.gap+width<=conf.width
	}

	pub fn add(&mut self, conf:&FlowConf, item:T, width:f32, height: f32) {
		let x=if self.items.len()==0 {
			0.0
		}

		else {
			self.width+conf.gap
		};

		self.items.push(FlowItem{item,width,height,x});
		self.width=x+width;

		if height>self.height {
			self.height=height;
		}
	}
}

#[derive(Debug)]
pub struct FlowBucket<T> {
	conf: FlowConf,
	lines: Vec<FlowLine<T>>
}

impl<T> FlowBucket<T> {
	pub fn new(w: f32, h: f32)->Self {
		Self {
			conf: FlowConf{
				width: w,
				height: h,
				gap: 0.,
				vgap: 0.
			},
			lines: vec![FlowLine::new()]
		}
	}

	fn current_line(&self)->&FlowLine<T> {
		let l=self.lines.len()-1;
		&self.lines[l]
	}

	pub fn add(&mut self, item:T, width:f32, height: f32) {
		if !self.current_line().can_fit(&self.conf,width) {
			self.lines.push(FlowLine::new())
		}

		let l=self.lines.len()-1;
		self.lines[l].add(&self.conf,item,width,height);
	}

	pub fn height(&self)->f32 {
		let mut h:f32=0.;

		for i in 0..self.lines.len() {
			if i!=0 {
				h+=self.conf.vgap;
			}
			h+=self.lines[i].height;
		}

		h
	}

	pub fn with_items<F>(&mut self, mut f:F)
			where F:FnMut(T,f32,f32,f32,f32) {
		let mut lines=vec![];
		lines.append(&mut self.lines);
		let mut y=0.;

		for mut line in lines {
			let mut items=vec![];
			items.append(&mut line.items);

			for item in items {
				f(item.item,item.x,y,item.width,item.height);
			}

			y+=line.height+self.conf.vgap;
		}
	}
}