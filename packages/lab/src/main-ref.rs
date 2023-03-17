use std::cell::RefCell;
use std::rc::Rc;
//use std::any::Any;
use std::ops::Deref;

/*struct Smart<T> {
	v: Rc<RefCell<T>>
}

impl<T> Smart<T> {
	fn new(v: T)->Smart<T> {
		Smart{v: Rc::new(RefCell::new(v))}
	}
}

impl<T> Deref for Smart<T> {
	type Target = T;

	fn deref(&self)->&Self::Target {
		&*self.v.borrow()
	} 
}*/

fn main() {
	let mut s=Rc::new(5);

//	let mut s=Box::new(5);

	*s=6;

	println!("v: {}",*s);
}