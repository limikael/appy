use appy_gl::{*};

pub struct AppProps {}

#[function_component]
fn app(_p: AppProps, _c: Elements)->Elements {
	apx!{
		<window>
			<rect x=0 y=0 w=100 h=100/>
		</window>
	}
}

fn main() {
	Appy::run(||apx!{<app/>});
}
