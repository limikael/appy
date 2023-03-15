use std::rc::Rc;
use std::cell::RefCell;

/*struct StructA {
	a: u32
}

struct StructB {
	a: u32
}

enum Enum {
	A(StructA),
	B(StructB)
}*/

enum Enum {
	A {
		a: u32
	},

	B {
		b: u32
	}
}

fn main() {
//	let e_refcell=Rc::new(RefCell::new(Enum::A(StructA{a:5})));
	let e_refcell=Rc::new(RefCell::new(Enum::A{a:5}));
	let mut e=e_refcell.borrow_mut();

	match *e {
		Enum::A{ref mut a} => {
			println!("{}",a);
			*a+=7;
			println!("{}",a);
		},
		_=>{}
	}

	match *e {
		Enum::A{ref mut a} => {
			println!("{}",a);
			*a+=7;
			println!("{}",a);
		},
		_=>{}
	}

/*	match *e {
		Enum::A(ref mut x) => {
			println!("{}",x.a);
			x.a+=7;
			println!("{}",x.a);
		},
		_=>{}
	}

	match *e {
		Enum::A(ref mut x) => {
			println!("{}",x.a);
			x.a+=7;
			println!("{}",x.a);
		},
		_=>{}
	}*/
}
