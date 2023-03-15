use appy::{Appy, Component, ComponentFragment, Typed, use_ref};
use appy_macros::{apx, component};
use std::rc::Rc;
use std::any::TypeId;

#[component]
struct Window {
}

impl Component for Window {
	fn render(&self)->ComponentFragment {
		println!("render window...");
		self.children.clone()
	}
}

#[component]
struct Rect {
	x: i32,
}

struct RectData {
	v: i32
}

impl Component for Rect {
	fn render(&self)->ComponentFragment {
		let rf=use_ref(||RectData{v: 32});

		println!("ref: {}",rf.borrow_mut().current.v);
		rf.borrow_mut().current.v=6;

		vec![]
	}
}

fn main() {
	Appy::run(apx!{
		<Window>
			<Rect x="1"/>
			<Rect x="2"/>
		</Window>
	});
}
