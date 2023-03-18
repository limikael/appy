use std::rc::Rc;
use appy::{*};

#[component]
struct App {}

impl Component for App {
	fn render(&self)->ComponentFragment {
		let (x,set_x)=use_state(||50);

		let inc=Rc::new(with_clone!([x,set_x],move||{
			set_x(*x+10)
		}));

		let dec=Rc::new(with_clone!([x,set_x],move||{
			set_x(*x-10)
		}));

//		let nop=Rc::new(||{});

		apx![
			<GlWindow>
				<Button x="10" y="10" w="100" h="50" on_click="inc" />
				<Button x="10" y="70" w="100" h="50" on_click="dec"/>
				<Rect x="*x" y="200" w="100" h="50" />
			</GlWindow>
		]
	}
}

fn main() {
	{
		let _self="hello";
	}
	Appy::run(apx!{<App/>});
}
