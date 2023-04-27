#![allow(clippy::needless_update)]

use appy::{*, hooks::*, components::*, types::*};

#[main_window]
pub fn app()->Elements {
	let v=use_state(||1);
	let hover_state=use_hover_state_ref();

	let on_button_click=rc_with_clone!([v],move||{
		//println!("set: {}",*v);

		v.set(*v+1);
	});

	let s=format!("Hello: {}",*v);
//	let s=format!("Hello...");
	let c=match *hover_state {
		HoverState::Normal=>0x808080,
		HoverState::Active=>0x404040,
		HoverState::Hover=>0xc0c0c0,
	};

	apx!{
		<blk top=Pc(25.0) height=Pc(25.0)>
			<text text=&*s align=Align::Center size=Pc(60.0) color=0xffffff/>
		</blk>
		<blk top=Pc(50.0) height=Pc(20.0) width=Pc(25.0)>
			<bg color=0xffffff/>
			<blk margin=Dp(2.0)>
				<bg color=c/>
				<text text="+1" align=Align::Center size=Pc(60.0) color=0xffffff/>
				<interaction hover_state_ref=hover_state on_click=on_button_click/>
			</blk>
		</blk>
	}
}