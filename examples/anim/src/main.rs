use std::rc::Rc;
use appy::{*, hooks::*, components::*, types::*};

#[derive_component(Default,ComponentBuilder,SnakeFactory)]
pub struct Button {
	text: String,
	on_click: Option<Rc<dyn Fn()>>,
}

#[function_component]
fn _button(props:Button)->Elements {
	let hover_state=use_hover_state_ref();
	let c=match *hover_state {
		HoverState::Normal=>0x808080,
		HoverState::Hover=>0xc0c0c0,
		HoverState::Active=>0x404040
	};

	apx! {
		<bg col=c/>
		<text text=&*props.text size=Pc(50.0) align=Align::Center/>
		<interaction on_click=props.on_click.unwrap() hover_state_ref=hover_state/>
	}
}

#[main_window]
pub fn main()->Elements {
	let x=use_spring(||0.0,SpringConf::DEFAULT);

	//println!("render");

	apx! {
		<Blk left=Dp(10.0) bottom=Dp(10.0) height=Dp(90.0) width=Dp(150.0)>
			<Button text="smooth"
					on_click=rc_with_clone!([x],move||x.target(0.0))/>
		</Blk>

		<Blk right=Dp(10.0) bottom=Dp(10.0) height=Dp(90.0) width=Dp(150.0)>
			<Button text="smooth"
					on_click=rc_with_clone!([x],move||x.target(100.0))/>
		</Blk>

		<Blk right=Dp(10.0) bottom=Dp(110.0) height=Dp(90.0) width=Dp(150.0)>
			<Button text="chop"
					on_click=rc_with_clone!([x],move||x.set(100.0))/>
		</Blk>

		<Blk right=Dp(50.0)>
			<Blk left=Pc(*x) width=Dp(50.0) height=Dp(50.0) top=Dp(50.0)>
				<Bg col=0xff0000/>
			</Blk>
		</Blk>
	}
}