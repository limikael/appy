use appy::{*};

#[derive(Clone,Default)]
pub struct App {}

#[function_component]
fn app(_p: App, _c: Elements)->Elements {
	let tx_ref=use_instance(||TextRenderer::new());
	let mut tx=tx_ref.borrow_mut();

	let window_ref=use_context::<GlWindowInstance>();
	let window=window_ref.borrow();

	tx.window_width=window.rect.w;
	tx.window_height=window.rect.h;

	tx.draw(&Rect{x:0, y:0, w:256, h:256},0xffff00);

	apx!()
}

fn main() {
	Appy::run(||apx!{
		<window>
			<blk left=Px(0.0) top=Px(0.0) width=Px(100.0) height=Px(100.0)>
				<bg col=0xff0000/>
			</blk>
			<app/>
		</window>
	});
}
