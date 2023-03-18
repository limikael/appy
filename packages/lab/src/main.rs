use std::rc::Rc;
use appy::{*};

struct AppProps {}

#[function_component]
fn app(p: AppProps, c: Elements)->Elements {
	println!("render hello!!");

	let (x,set_x)=use_state(||50);

	let inc=Rc::new(with_clone!([x,set_x],move||{
		set_x(*x+10)
	}));

	let dec=Rc::new(with_clone!([x,set_x],move||{
		set_x(*x-10)
	}));

//		let nop=Rc::new(||{});

	let mut xx=*x;

	let res:Elements=apx![
		<gl_window>
			<button x="10" y="10" w="100" h="50" on_click="Rc::new(||{println!()})" />
			<button x="10" y="70" w="100" h="50" on_click="Rc::new(||{})"/>
			<rect x="xx" y="200" w="100" h="50" />
		</gl_window>
	];

	xx=2;

	res
}

fn main() {
	Appy::run(apx!{<app/>});
}
