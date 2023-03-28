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

#[appy_main]
fn app()->Elements {
	let trigger=use_dirty_trigger();
	let model_ref=use_instance(||CalculatorModel::new(trigger.clone()));
	let on_click=cb_p_with_clone!([model_ref],move|c:char|{
		model_ref.borrow_mut().input(c);
	});

	let model=model_ref.borrow();

	apx!(
		<window title="Calculator".to_string()
				desktop_init_size=(360,480)>
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
		</window>
	)
}
