use appy::{*};

mod calculator_model;
use calculator_model::{*};

#[derive(Clone, Default)]
pub struct Button {
	left: Dim,
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
		<blk left=p.left height=Pc(100.0) width=Pc(25.0)>
			<blk left=Pc(10.0) top=Pc(10.0) right=Pc(10.0) bottom=Pc(10.0)>
				<interaction on_click=on_click hover_state_ref=hover_state_ref/>
				<bg col=col/>
				<text text=p.id.to_string() size=Pc(65.0) align=Align::Center col=0x000000/>
			</blk>
		</blk>
	)
}

#[derive(Clone, Default)]
pub struct ButtonRow {
	top: Dim,
	on_click: CbP<char>,
	ids: Vec<char>
}

#[function_component]
fn button_row(p: ButtonRow, _c: Elements)->Elements {
	apx!(
		<blk height=Pc(20.0) top=p.top>
			<button left=Pc(0.0) on_click=p.on_click.clone() id=p.ids[0]/>
			<button left=Pc(25.0) on_click=p.on_click.clone() id=p.ids[1]/>
			<button left=Pc(50.0) on_click=p.on_click.clone() id=p.ids[2]/>
			<button left=Pc(75.0) on_click=p.on_click.clone() id=p.ids[3]/>
		</blk>
	)
}

#[derive(Default)]
pub struct AppProps {}

#[function_component]
fn app(_p: AppProps, _c: Elements)->Elements {
	//println!("render...");

	let trigger=use_dirty_trigger();
	let model_ref=use_instance(||CalculatorModel::new(trigger.clone()));
	let on_click=cb_p_with_clone!([model_ref],move|c:char|{
		model_ref.borrow_mut().input(c);
	});

	let model=model_ref.borrow();

	apx!(
		<window title="Calculator".to_string()
				init_width=360
				init_height=480>
			<blk height=Pc(25.0) top=Pc(0.0)>
				<bg col=0x3C1518/>
				<blk left=Pc(5.0) right=Pc(5.0)>
					<text align=Align::Right text=model.get_display_value() size=Pc(50.0)/>
				</blk>
			</blk>
			<blk top=Pc(25.0)>
				<bg col=0x69140E/>
				<blk left=Pc(2.0) top=Pc(2.0) right=Pc(2.0) bottom=Pc(2.0)>
					<button_row top=Pc(0.0)  on_click=on_click.clone() ids=vec!['C','«','%','/']/>
					<button_row top=Pc(20.0) on_click=on_click.clone() ids=vec!['7','8','9','*']/>
					<button_row top=Pc(40.0) on_click=on_click.clone() ids=vec!['4','5','6','-']/>
					<button_row top=Pc(60.0) on_click=on_click.clone() ids=vec!['1','2','3','+']/>
					<button_row top=Pc(80.0) on_click=on_click.clone() ids=vec!['±','0','.','=']/>
				</blk>
			</blk>
		</window>
	)
}

fn main() {
	Appy::run(||apx!{<app/>});
}
