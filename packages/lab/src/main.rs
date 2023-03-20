use appy_gl::{*};

pub struct AppProps {}

#[function_component]
fn app(_p: AppProps, _c: Elements)->Elements {
	apx!{
		<window>
			<abs x=10 y=10 w=100 h=100>
				<rect col=0x00ff00/>
				<abs x=10 y=10 w=10 h=10>
					<rect col=0xff0000/>
				</abs>

				<abs x=10 y=30 w=10 h=10>
					<rect col=0xff0000/>
				</abs>
			</abs>
		</window>
	}
}

fn main() {
	Appy::run(||apx!{<app/>});
}
