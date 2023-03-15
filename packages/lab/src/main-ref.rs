use std::cell::RefCell;
use std::rc::Rc;
use std::any::Any;

fn main() {
//	let x:Rc<dyn Any>=Rc::new(123);
//	let v=x.downcast::<i32>().unwrap();

//	let x:Rc<RefCell<dyn Any>>=Rc::new(RefCell::new(1));
//	let v=x.downcast::<RefCell<i32>>().unwrap();

	let x:Rc<dyn Any>=Rc::new(RefCell::new(1));
	let v:Rc<RefCell<i32>>=x.downcast::<RefCell<i32>>().unwrap();

	println!("awfe {}",v.borrow());
}