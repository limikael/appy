use appy_gl::{*};
use std::rc::Rc;

pub struct AppProps {}

#[function_component]
fn app(_p: AppProps, _c: Elements)->Elements {
	let on_click=Rc::new(move||{
		println!("click!!!")
	});

	apx!{
		<window>
			<rel x=0 y=75 w=33 h=25>
				<rect col=0x808000/>
			</rel>
			<rel x=33 y=75 w=34 h=25>
				<rect col=0x008080/>
			</rel>
			<rel x=67 y=75 w=33 h=25>
				<rect col=0x800080/>
			</rel>
			<abs x=10 y=10 w=100 h=100>
				<rect col=0x00ff00/>
				<abs x=10 y=10 w=30 h=30>
					<interactive on_mouse_down=on_click>
						<rect col=0xff0000/>
					</interactive>
				</abs>

				<abs x=10 y=50 w=30 h=30>
					<rect col=0xff0000/>
				</abs>
			</abs>
		</window>
	}
}

fn main() {
	Appy::run(||apx!{<app/>});
}
