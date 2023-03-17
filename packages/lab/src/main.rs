use std::rc::Rc;
use appy::{*};

#[component]
struct App {}

impl Component for App {
	fn render(&self)->ComponentFragment {
		let (x,set_x)=use_state(||50);
		//println!("render, x={}",*x.borrow());

		let inc={
			let x=x.clone();
			let set_x=set_x.clone();
			Rc::new(move||{
				let v=*x.borrow()+10;
				set_x(v);	
			})
		};

		let dec={
			let x=x.clone();
			let set_x=set_x.clone();
			Rc::new(move||{
				let v=*x.borrow()-10;
				set_x(v);	
			})
		};

		let xv=*x.borrow();

		apx!{
			<GlWindow>
				<Button x="10" y="10" w="100" h="50" on_click="inc"/>
				<Button x="10" y="70" w="100" h="50" on_click="dec"/>
				<Rect x="xv" y="200" w="100" h="50" />
			</GlWindow>
		}
	}
}

fn main() {
	Appy::run(apx!{<App/>});
}
