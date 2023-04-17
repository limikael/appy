use std::rc::Rc;
use appy::core::hooks::use_spring;
use appy::core::hooks::SpringConf;
use appy::core::element::*;
use appy::components::blk::*;
use appy::components::bg::*;
use appy::components::text::*;
use appy::components::interaction::*;
use appy::{main_window,apx,component};
use appy::components::blk::Dim::*;
use appy::rc_with_clone;

#[component]
pub struct Button {
	text: String,
	on_click: Option<Rc<dyn Fn()>>
}

impl Element for Button {
	fn render(self:ElementWrap<Self>)->Elements {
		let hover_state=use_hover_state_ref();
		let c=match *hover_state {
			HoverState::Normal=>0x808080,
			HoverState::Hover=>0xc0c0c0,
			HoverState::Active=>0x404040
		};

		apx! {
			<bg col=c/>
			<text text=self.text size=Pc(50.0) align=Align::Center/>
			<interaction on_click=self.on_click.unwrap() hover_state_ref=hover_state/>
		}
	}
}

#[main_window]
pub fn app()->Elements {
	let x=use_spring(||0.0,SpringConf::DEFAULT);

	//println!("render");

	apx! {
		<blk left=Dp(10.0) bottom=Dp(10.0) height=Dp(90.0) width=Dp(150.0)>
			<button text="smooth".to_string()
					on_click=rc_with_clone!([x],move||x.target(0.0))/>
		</blk>

		<blk right=Dp(10.0) bottom=Dp(10.0) height=Dp(90.0) width=Dp(150.0)>
			<button text="smooth".to_string()
					on_click=rc_with_clone!([x],move||x.target(100.0))/>
		</blk>

		<blk right=Dp(10.0) bottom=Dp(110.0) height=Dp(90.0) width=Dp(150.0)>
			<button text="chop".to_string()
					on_click=rc_with_clone!([x],move||x.set(100.0))/>
		</blk>

		<blk right=Dp(50.0)>
			<blk left=Pc(*x) width=Dp(50.0) height=Dp(50.0) top=Dp(50.0)>
				<bg col=0xff0000/>
			</blk>
		</blk>
	}
}