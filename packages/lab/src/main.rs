use std::rc::Rc;
use appy::{*};
use closure::closure;

#[component]
struct App {}

impl Component for App {
	fn render(&self)->ComponentFragment {
		let (x,set_x)=use_state(||50);

		let inc=Rc::new(closure!(clone x, clone set_x, ||{
			set_x(*x+20)
		}));

		let dec=Rc::new(closure!(clone x, clone set_x, ||{
			set_x(*x-20)
		}));

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
