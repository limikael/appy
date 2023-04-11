use std::rc::Rc;
use std::cmp::min;

#[derive(Clone)]
pub struct CalculatorModel {
	accum: f64,
	input: String,
	current_op: Option<char>
}

fn but_last(s:String)->String {
	let cv:Vec<char>=s.to_string().chars().collect();
	let split=cv.split_last().unwrap().1;
	split.iter().collect()
}

fn calc_fmt(v:f64)->String {
	let log=min(8,v.abs().log10().floor() as usize);
	let mut s=format!("{:.*}",8-log,v);

	while s.contains(".") &&
			s.chars().last().unwrap()=='0' {
		s=but_last(s);
	}

	if s.chars().last().unwrap()=='.' {
		s=but_last(s);
	}

	s
}

impl CalculatorModel {
	pub fn new()->CalculatorModel {
		CalculatorModel {
			accum: 0.0,
			input: "".to_string(),
			current_op: None
		}
	}

	fn get_input(&self)->f64 {
		if self.input.is_empty() {
			return 0.0;
		}

		self.input.parse::<f64>().unwrap()
	}

	pub fn action(&self, c:char)->Self {
		let mut res=self.clone();
		res.input(c);
		res
	}

	pub fn input(&mut self, c:char) {
		//println!("input: {}",c);

		match c {
			'0'..='9'|'.'=> 'b:{
				if self.input.len()>=10 {
					break 'b;
				}

				if c=='.' && self.input.contains('.') {
					break 'b;
				}

				self.input=format!("{}{}",self.input,c);
			},

			'+'|'-'|'*'|'/'=>{
				self.apply_current_op();
				self.current_op=Some(c);
			},

			'C'=>{
				self.accum=0.0;
				self.current_op=None;
				self.input="".to_string();
			},

			'='=>{
				self.apply_current_op();
				self.input="".to_string();
			},

			'«'=>{
				self.input="".to_string();
			},

			'%'=>{
				match (self.input.as_str(), self.accum) {
					("",_)=>self.accum*=0.01,
					(_,a) if a==0.0=>self.input=calc_fmt(self.get_input()*0.01),
					(_,_)=>self.input=calc_fmt(self.get_input()*self.accum*0.01),
				}
			},

			'±'=>{
				match self.input.as_str() {
					""=>self.accum=-self.accum,
					_=>self.input=calc_fmt(-self.get_input())
				}
			}

			_=>panic!("unexpected input")
		};
	}

	fn apply_current_op(&mut self) {
		if !self.input.is_empty() {
			match self.current_op {
				Some('+')=>self.accum+=self.get_input(),
				Some('-')=>self.accum-=self.get_input(),
				Some('*')=>self.accum*=self.get_input(),
				Some('/')=>self.accum/=self.get_input(),
				None=>self.accum=self.get_input(),
				_=>panic!("smething is strange")
			}
		}

		self.current_op=None;
		self.input="".to_string();
	}

	pub fn get_display_value(&self)->String {
		if !self.input.is_empty() {
			return self.input.clone();
		}

		calc_fmt(self.accum)
	}
}