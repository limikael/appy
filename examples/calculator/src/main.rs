// TODO: is there a way to put that in the macro itself, or to avoid
// putting `..Default::default()` at the end when all parameters exist?
#![allow(clippy::needless_update)]

use std::rc::Rc;
use appy::{*, hooks::*, components::*, types::*};

mod calculator_model;
use calculator_model::CalculatorModel;

#[derive_component(Default,ComponentBuilder,SnakeFactory)]
pub struct Button {
	on_click: Option<Rc<dyn Fn(char)>>,
	id: char
}

#[function_component]
fn _button(props:Button)->Elements {
	let hover_state_ref=use_hover_state_ref();
	let self_on_click=props.on_click.as_ref().unwrap().clone();
	let self_id=props.id.clone();
	let on_click=rc_with_clone!([],move||{
		(self_on_click)(self_id)
	});

	let color=match *hover_state_ref {
		HoverState::Normal=>0xD58936,
		HoverState::Hover=>0xDEA260,
		HoverState::Active=>0xB36F25
	};

	apx!(
		<blk left=Pc(10.0) top=Pc(10.0) right=Pc(10.0) bottom=Pc(10.0)>
			<interaction on_click=on_click hover_state_ref=hover_state_ref/>
			<bg color=color/>
			<text text=&*props.id.to_string() size=Pc(65.0) align=Align::Center color=0x000000/>
		</blk>
	)
}

#[derive_component(Default,ComponentBuilder,SnakeFactory)]
pub struct ButtonBg {
    pub on_click: Option<Rc<dyn Fn()>>,
	pub normal: u32,
	pub active: u32,
	pub hover: u32
}

#[function_component]
fn _button_bg(props:ButtonBg)->Elements {
	let hover_state=use_hover_state_ref();
	//println!("state: {:?}",*hover_state);

	let c=match *hover_state {
		HoverState::Normal=>props.normal,
		HoverState::Hover=>props.hover,
		HoverState::Active=>props.active
	};

	apx!{
		<bg color=c/>
		<interaction on_click=props.on_click.unwrap() hover_state_ref=hover_state/>
	}
}

#[main_window]
fn app()->Elements {
	let model=use_reducer(CalculatorModel::action,CalculatorModel::new);
	let show_info=use_state(||false);

	let on_click=rc_with_clone!([model],move|c:char|{
		model.dispatch(c);
	});

	let on_info_click=rc_with_clone!([show_info],move||{
		show_info.set(!*show_info);
	});

	apx!(
		<blk height=Pc(25.0) top=Pc(0.0)>
			<bg color=0x3C1518/>
			<blk left=Pc(5.0) right=Pc(5.0)>
				<text align=Align::Right 
						text=&*model.get_display_value() size=Pc(50.0) color=0xffffff/>
			</blk>
		</blk>
		<blk top=Pc(25.0)>
			<bg color=0x69140E/>
			<blk left=Pc(2.0) top=Pc(2.0) right=Pc(2.0) bottom=Pc(2.0)>
				<grid rows=5 cols=4>
					{"C«%/789*456-123+±0.=".chars().into_iter().flat_map(|c| {
						apx!{
							<button id=c on_click=on_click.clone() />
						}
					}).collect()}
				</grid>
			</blk>
		</blk>
		<blk top=Pc(5.0) left=Pc(5.0) width=Pc(10.0) height=Pc(10.0)>
			<button_bg normal=0x000000 active=0x404040 hover=0x808080
					on_click=on_info_click.clone()/>
			<text text="i" size=Pc(100.0) align=Align::Center color=0xffffff/>
		</blk>
		{if *show_info {
			apx!(
				<blk top=Pc(10.0) left=Pc(10.0) right=Pc(10.0) bottom=Pc(10.0)>
					<bg color=0x102030/>
					<blk bottom=Pc(10.0) width=Pc(50.0) height=Pc(10.0)>
						<button_bg normal=0x0000f0 active=0x4040f0 hover=0x8080f0
								on_click=on_info_click.clone()/>
						<text text="Ok"
								align=Align::Center size=Pc(100.0) color=0xffffff/>
					</blk>
					<text text="This is a little calculator..."
							align=Align::Center size=Dp(32.0) color=0xffffff/>
				</blk>
			)
		} else {apx!()}}
	)
}
