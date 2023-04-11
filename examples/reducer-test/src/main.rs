use appy::{*};

#[derive(Clone)]
enum Action {
	Add,
	Sub
}

#[derive(Clone)]
struct AppState {
	v: i32
}

impl AppState {
	fn reducer(&self, a:Action)->Self {
		match a {
			Action::Add=>Self{v: self.v+1},
			Action::Sub=>Self{v: self.v-1}
		}
	}
}

#[derive(Default,Clone)]
pub struct TextButton {
	text: String,
	on_click: Cb
}

#[function_component]
pub fn text_button(p:TextButton, _c:Elements)->Elements {
	let hover_state=use_hover_state_ref();
	let c=match *hover_state {
		HoverState::Normal=>0x808080,
		HoverState::Active=>0x404040,
		HoverState::Hover=>0xc0c0c0,
	};

	apx! {
		<bg col=c/>
		<text text=p.text align=Align::Center size=Pc(60.0)/>
		<interaction hover_state_ref=Some(hover_state) on_click=p.on_click/>
	}
}

#[main_window]
pub fn app()->Elements {
	let state=use_reducer(AppState::reducer,||AppState{v:1});

	let s=format!("Value: {}",(*state).v);

	apx!{
		<blk top=Pc(25.0) height=Pc(25.0)>
			<text text=s.to_string() align=Align::Center size=Pc(60.0)/>
		</blk>
		<grid cols=2>
			<blk top=Pc(50.0) height=Pc(20.0) width=Pc(50.0)>
				<text_button text="-1".to_string() 
					on_click=cb_with_clone!([state],move||{
						state.dispatch(Action::Sub);
					})
				/>
			</blk>
			<blk top=Pc(50.0) height=Pc(20.0) width=Pc(50.0)>
				<text_button text="+1".to_string() 
					on_click=cb_with_clone!([state],move||{
						state.dispatch(Action::Add);
					})
				/>
			</blk>
		</grid>
	}
}