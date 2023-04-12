use appy::{*};

mod calculator_model;
use calculator_model::{*};

#[derive(Clone, Default)]
pub struct Button {
	on_click: CbP<char>,
	id: char
}

#[function_component]
fn button(p: Button, _c: Elements)->Elements {
	let hover_state_ref=use_hover_state_ref();
	let on_click=cb_with_clone!([p],move||{
		(p.on_click)(p.id)
	});

	let col=match *hover_state_ref {
		HoverState::Normal=>0xD58936,
		HoverState::Hover=>0xDEA260,
		HoverState::Active=>0xB36F25
	};

	apx!(
		<blk left=Pc(10.0) top=Pc(10.0) right=Pc(10.0) bottom=Pc(10.0)>
			<interaction on_click=on_click hover_state_ref=Some(hover_state_ref)/>
			<bg col=col/>
			<text text=p.id.to_string() size=Pc(65.0) align=Align::Center col=0x000000/>
		</blk>
	)
}

#[derive(Clone, Default)]
pub struct ButtonBg {
    pub on_click: Cb,
	pub normal: u32,
	pub active: u32,
	pub hover: u32
}

#[function_component]
pub fn button_bg(p:ButtonBg, _c:Elements)->Elements {
	let hover_state=use_hover_state_ref();
	//println!("state: {:?}",*hover_state);

	let c=match *hover_state {
		HoverState::Normal=>p.normal,
		HoverState::Hover=>p.hover,
		HoverState::Active=>p.active
	};

	apx!{
		<bg col=c/>
		<interaction on_click=p.on_click hover_state_ref=Some(hover_state)/>
	}
}

#[main_window]
fn app()->Elements {
	let model=use_reducer(CalculatorModel::action,CalculatorModel::new);
	let show_info=use_state(||false);

	let on_click=cb_p_with_clone!([model],move|c:char|{
		model.dispatch(c);
	});

	let on_info_click=cb_with_clone!([show_info],move||{
		show_info.set(!*show_info);
	});

	apx!(
		<blk height=Pc(25.0) top=Pc(0.0)>
			<bg col=0x3C1518/>
			<blk left=Pc(5.0) right=Pc(5.0)>
				<text align=Align::Right text=model.get_display_value() size=Pc(50.0)/>
			</blk>
		</blk>
		<blk top=Pc(25.0)>
			<bg col=0x69140E/>
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
			<text text="i".to_string() size=Pc(100.0) align=Align::Center/>
		</blk>
		{if *show_info {
			apx!(
				<blk top=Pc(10.0) left=Pc(10.0) right=Pc(10.0) bottom=Pc(10.0)>
					<bg col=0x102030/>
					<blk bottom=Pc(10.0) width=Pc(50.0) height=Pc(10.0)>
						<button_bg normal=0x0000f0 active=0x4040f0 hover=0x8080f0
								on_click=on_info_click.clone()/>
						<text text="Ok".to_string() align=Align::Center/>
					</blk>
					<text text="This is a little calculator...".to_string() align=Align::Center/>
				</blk>
			)
		} else {apx!()}}
	)
}
