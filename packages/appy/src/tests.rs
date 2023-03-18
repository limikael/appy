use crate::{*};
use std::rc::Rc;
use std::any::Any;

struct MyStruct{
	i: i32
}

fn mycomp(p: &MyStruct) {
	println!("****** mycomp {:?}",p.i);
}

struct Other{
	x: i32
}

fn othercomp(p: &Other) {
	println!("********* other {:?}",p.x);
}

trait Elem {
	fn render(&self);
}

struct El<T> {
	props: T,
	renderer: fn(&T)
}

impl<T: 'static> El<T> {
	pub fn call_render(&self) {
		(&self.renderer)(&self.props)
	}

	pub fn create(renderer: fn(&T), props:T)->Rc<dyn Elem> {
		Rc::new(Self{renderer,props})
	}
}

impl<T: 'static> Elem for El<T> {
	fn render(&self) {
		self.call_render()
	}
}

type Elements=Vec<Rc<dyn Elem>>;

#[test]
fn test() {
	let v=vec![
		El::create(mycomp,MyStruct{i: 123}),
		El::create(othercomp,Other{x: 321})
	];

/*	let a:Rc<dyn Elem>=Rc::new(El::create(mycomp,MyStruct{i: 123}));
	let b:Rc<dyn Elem>=Rc::new(El::create(othercomp,Other{x: 321}));

	let v=vec![a,b];

	v[0].render();
	v[1].render();*/
}