use appy::core::hooks::use_spring;
use appy::core::hooks::SpringConf;
use appy::core::element::Elements;
use appy::components::blk::*;
use appy::components::bg::*;
use appy::components::text::*;
use appy::components::interaction::*;
use appy::{main_window,apx,function_component};
use appy::components::blk::Dim::*;
use appy::utils::cb::Cb;
use appy::cb_with_clone;

#[derive(Default)]
pub struct Button {
	left: Dim,
	top: Dim,
	text: String,
	on_click: Cb
}

#[function_component]
pub fn button(p:Button, _c:Elements)->Elements {
	let hover_state=use_hover_state_ref();
	let c=match *hover_state {
		HoverState::Normal=>0x808080,
		HoverState::Hover=>0xc0c0c0,
		HoverState::Active=>0x404040
	};

	apx! {
		<blk left=p.left top=p.top width=Dp(100.0) height=Dp(32.0)>
			<bg col=c/>
			<text text=p.text size=Pc(100.0) align=Align::Center/>
			<interaction on_click=p.on_click hover_state_ref=Some(hover_state)/>
		</blk>
	}
}

#[main_window]
pub fn app()->Elements {
	let x=use_spring(||0.0,SpringConf::DEFAULT);

	//println!("render");

	apx! {
		<button top=Pc(0.0) left=Pc(0.0) text="soft".to_string()
				on_click=cb_with_clone!([x],move||x.target(0.0))/>
		<button top=Pc(0.0) left=Pc(75.0) text="soft".to_string()
				on_click=cb_with_clone!([x],move||x.target(75.0))/>
		<button top=Pc(10.0) left=Pc(75.0) text="hard".to_string()
				on_click=cb_with_clone!([x],move||x.set(75.0))/>
		<blk left=Pc(*x) width=Dp(32.0) height=Dp(32.0)>
			<bg col=0xff0000/>
		</blk>
	}
}