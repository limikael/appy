//use crate::types::{Align,VAlign};
use crate::{types::*,components::*};

pub struct FlowElement {
	pub width: f32,
	pub height: f32,
	pub children: Elements,
	pub key: Option<String>
}

//#[derive(Debug)]
pub struct FlowConf {
	pub width: f32,
	pub height: f32,
	pub gap: f32,
	pub vgap: f32,
	/*pub align: Align,
	pub valign: VAlign*/
}

//#[derive(Debug)]
struct FlowItem {
	element: FlowElement,
	x: f32
}

//#[derive(Debug)]
struct FlowLine {
	items: Vec<FlowItem>,
	width: f32,
	height: f32,
}

impl FlowLine {
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

	pub fn add(&mut self, conf:&FlowConf, element:FlowElement) {
		let x=if self.items.len()==0 {
			0.0
		}

		else {
			self.width+conf.gap
		};

		self.width=x+element.width;
		if element.height>self.height {
			self.height=element.height;
		}

		self.items.push(FlowItem{element,x});
	}
}

//#[derive(Debug)]
pub struct FlowBucket {
	conf: FlowConf,
	lines: Vec<FlowLine>
}

impl FlowBucket {
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

	fn current_line(&self)->&FlowLine {
		let l=self.lines.len()-1;
		&self.lines[l]
	}

	pub fn add(&mut self, element:FlowElement) {
		if !self.current_line().can_fit(&self.conf,element.width) {
			self.lines.push(FlowLine::new())
		}

		let l=self.lines.len()-1;
		self.lines[l].add(&self.conf,element);
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

	pub fn create_blocks(&mut self)->Elements {
        let mut elements:Elements=vec![];

		let mut lines=vec![];
		lines.append(&mut self.lines);
		let mut y=0.;

		for mut line in lines {
			let mut items=vec![];
			items.append(&mut line.items);

			for item in items {
	            elements.push(blk()
	                .left(item.x)
	                .top(y)
	                .width(item.element.width)
	                .height(item.element.height)
	                .children(item.element.children)
	                .key_option(item.element.key)
	            )
			}

			y+=line.height+self.conf.vgap;
		}

		elements
	}
}