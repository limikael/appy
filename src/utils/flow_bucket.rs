//use crate::types::{Align,VAlign};
use crate::{types::*,components::*};
use std::mem::take;

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
	pub align: Align,
	pub valign: VAlign
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
	pub fn flow(elements:Vec<FlowElement>, conf:FlowConf)->Elements {
		let mut flow_bucket=FlowBucket::new(conf);
		for element in elements {
			flow_bucket.add(element)
		}

		flow_bucket.create_blocks()
	}

	pub fn new(conf: FlowConf)->Self {
		Self {
			conf,
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
		let mut y=match self.conf.valign {
			VAlign::Top=>0.,
			VAlign::Middle=>(self.conf.height-self.height())/2.0,
			VAlign::Bottom=>self.conf.height-self.height()
		};

		for mut line in take(&mut self.lines) {
			let line_start=match self.conf.align {
				Align::Left=>0.,
				Align::Center=>(self.conf.width-line.width)/2.0,
				Align::Right=>self.conf.width-line.width
			};

			for item in take(&mut line.items) {
	            elements.push(blk()
	                .left(line_start+item.x)
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