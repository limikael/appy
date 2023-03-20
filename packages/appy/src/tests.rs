use crate::{*};
//use std::rc::Rc;
//use std::any::Any;

/*pub struct HelloProps {
	x: i32
}

#[function_component]
pub fn hello(p:HelloProps, c:Elements)->Elements {
	println!("render hello...");

	apx!{}	
}

#[test]
fn test() {
	println!("** test");
	let e=Element::create(hello,HelloProps{x: 123},vec![]);

	let v=e.render();

//	e.render();
//	*e.render();

//	let x:Elements=apx!{
//		<hello x="5"/>
//	};
}
*/

/*trait MyTrait {
	fn consume_trait(self: Box<Self>);
}

struct MyStruct {} 

impl MyTrait for MyStruct {
	fn consume_trait(self: Box<Self>) {

	}
}

impl MyStruct {
	fn consume_struct(self) {

	}

	fn other(&self) {

	}
}

#[test]
fn test2() {
	let a:Box<dyn MyTrait>=Box::new(MyStruct{});
	a.consume_trait();

//	let b:Box<dyn MyTrait>=a;
//	let c:Box<dyn Any+'static>=a;

//	let pv=a.downcast::<MyStruct>();

//	a.consume_trait();
//	a.consume_struct();

//	a.other();

//	let a:Box<dyn MyTrait>=Box::new(MyStruct{});
}*/

struct HelloProps {
	x: i32,
	y: i32
}

#[function_component]
fn hello(p:HelloProps, c:Elements) -> Elements {
	vec![]
}

#[test]
fn test2() {
//	let t:Elements=apx!{};
	let t:Elements=apx!{<hello x=5+10 y=123 />};
}