use std::rc::Rc;
use appy::{*};

pub struct AppProps {}

#[function_component]
fn app(_p: AppProps, _c: Elements)->Elements {
	println!("render hello!!");

	let (x,set_x)=use_state(||50);

	let inc=Rc::new(with_clone!([x,set_x],move||{
		set_x(*x+10)
	}));

	let dec=Rc::new(with_clone!([x,set_x],move||{
		set_x(*x-10)
	}));

	let res:Elements=apx![
		<gl_window>
			<button x="10" y="10" w="100" h="50" on_click="inc" />
			<button x="10" y="70" w="100" h="50" on_click="dec"/>
			<rect x="*x" y="200" w="100" h="50" />
		</gl_window>
	];

	res
}

fn main() {
	Appy::run(||apx!{<app/>});
}
