use appy_gl::{*};
use std::rc::Rc;

use Dim::{*};

/*#[derive(Clone)]
struct Cb {
	func: Rc<dyn Fn(String)>
}

/*impl std::ops::Deref for Cb {
    type Target = dyn Fn();

    fn deref(&self) -> &dyn Fn() {
        &self.func
    }
}*/

impl Cb {
/*	fn new(f: &dyn Fn(String))->Self {
		Self {
			func: Rc::new(f)
		}
	}*/
}

impl Default for Cb {
	fn default()->Self {
		Self {
			func: Rc::new(|s|{})
		}
	}
}*/

#[derive(Clone)]
pub struct Button {
	left: Dim,
	on_click: Rc<dyn Fn(char)>,
	id: char
}

impl Default for Button {
	fn default()->Self {
		Self {
			on_click: Rc::new(|_s|{}),
			id: ' ',
			left: None
		}
	}
}

#[function_component]
fn button(p: Button, _c: Elements)->Elements {
	let on_click=Rc::new(with_clone!([p],move||{
		(p.on_click)(p.id)
	}));

	apx!(
		<blk left=p.left height=Pc(100.0) width=Pc(25.0)>
			<blk left=Pc(10.0) top=Pc(10.0) right=Pc(10.0) bottom=Pc(10.0)>
				<interactive on_mouse_down=on_click />
				<rect col=0xc0c0ff/>
			</blk>
		</blk>
	)
}

#[derive( Clone)]
pub struct ButtonRow {
	top: Dim,
	on_click: Rc<dyn Fn(char)>,
	ids: Vec<char>
}

impl Default for ButtonRow {
	fn default()->Self {
		Self {
			on_click: Rc::new(|_s|{}),
			top: None,
			ids: vec![]
		}
	}
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
	let on_click=Rc::new(move|s:char|{
		println!("click!!!,{}",s)
	});

	apx!(
		<window>
			<blk top=Pc(25.0)>
				<rect col=0x202020/>
				<blk left=Pc(2.0) top=Pc(2.0) right=Pc(2.0) bottom=Pc(2.0)>
					<button_row top=Pc(0.0) on_click=on_click.clone() ids=vec!['a','b','c','d']/>
					<button_row top=Pc(20.0) on_click=on_click.clone() ids=vec!['a','f','c','d']/>
					<button_row top=Pc(40.0) on_click=on_click.clone() ids=vec!['f','b','c','d']/>
					<button_row top=Pc(60.0) on_click=on_click.clone() ids=vec!['a','x','c','d']/>
					<button_row top=Pc(80.0) on_click=on_click.clone() ids=vec!['x','b','c','d']/>
				</blk>
			</blk>
		</window>
	)
}

fn main() {
	Appy::run(||apx!{<app/>});
}
