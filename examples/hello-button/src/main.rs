use appy::{*};

#[main_window]
pub fn app()->Elements {
	let (v,set_v)=use_state(||1);
	let hover_state=use_hover_state_ref();

	let on_button_click=cb_with_clone!([v,set_v],move||{
		set_v(*v+1);
	});

	let s=format!("Hello: {}",*v);
	let c=match *hover_state {
		HoverState::Normal=>0x808080,
		HoverState::Active=>0x404040,
		HoverState::Hover=>0xc0c0c0,
	};

	apx!{
		<blk top=Pc(25.0) height=Pc(25.0)>
			<text text=s.to_string() align=Align::Center size=Pc(60.0)/>
		</blk>
		<blk top=Pc(50.0) height=Pc(20.0) width=Pc(25.0)>
			<bg col=c/>
			<text text="+1".to_string() align=Align::Center size=Pc(60.0)/>
			<interaction hover_state_ref=Some(hover_state) on_click=on_button_click/>
		</blk>
	}
}