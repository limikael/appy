use appy::{GlWindow, Appy, Component, ComponentFragment, Typed, use_instance, use_ref};
use appy_macros::{apx, component};
use std::rc::Rc;
use std::any::TypeId;

/*#[component]
struct Window {
}

impl Component for Window {
	fn render(&self)->ComponentFragment {
		println!("render window...");
		self.children.clone()
	}
}*/

#[component]
struct Rect {
	x: i32,
}

struct RectData {
	v: i32
}

impl Component for Rect {
	fn render(&self)->ComponentFragment {
		let rd_ref=use_instance(||RectData{v: 32});
		let mut rd=rd_ref.borrow_mut();
		//println!("instance_data: {}",rd.v);
		rd.v=6;

		let r_ref=use_ref(||123);
		let mut r=r_ref.borrow_mut();

		//println!("ref_data: {}",r.current);
		r.current=321;

		/*let trigger=use_trigger();
		trigger();*/

		vec![]
	}
}

fn main() {
	Appy::run(apx!{
		<GlWindow>
			<Rect x="1"/>
			<Rect x="2"/>
		</GlWindow>
	});
}
