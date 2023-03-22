use appy::{*};

#[derive(Clone, Default)]
pub struct Button {
	left: Dim,
	on_click: CbP<char>,
	id: char
}

#[function_component]
fn button(p: Button, _c: Elements)->Elements {
	let on_click=cb_with_clone!([p],move||{
		(p.on_click)(p.id)
	});

	apx!(
		<blk left=p.left height=Pc(100.0) width=Pc(25.0)>
			<blk left=Pc(10.0) top=Pc(10.0) right=Pc(10.0) bottom=Pc(10.0)>
				<interactive on_mouse_down=on_click />
				<bg col=0xc0c0ff/>
				<text text=p.id.to_string() size=50.0 align=Align::Center col=0x000000/>
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
	let on_click=cb_p_with_clone!([],move|s:char|{
		println!("click!!!,{}",s)
	});

	apx!(
		<window>
			<blk height=Pc(25.0) top=Pc(0.0) left=Pc(5.0) right=Pc(5.0)>
				<text align=Align::Right text="123".to_string() size=100.0/>
			</blk>
			<blk top=Pc(25.0)>
				<bg col=0x202020/>
				<blk left=Pc(2.0) top=Pc(2.0) right=Pc(2.0) bottom=Pc(2.0)>
					<button_row top=Pc(0.0)  on_click=on_click.clone() ids=vec!['C','±','%','/']/>
					<button_row top=Pc(20.0) on_click=on_click.clone() ids=vec!['7','8','9','*']/>
					<button_row top=Pc(40.0) on_click=on_click.clone() ids=vec!['4','5','6','-']/>
					<button_row top=Pc(60.0) on_click=on_click.clone() ids=vec!['1','2','3','+']/>
					<button_row top=Pc(80.0) on_click=on_click.clone() ids=vec![' ','0','.','=']/>
				</blk>
			</blk>
		</window>
	)
}

fn main() {
	Appy::run(||apx!{<app/>});
}
