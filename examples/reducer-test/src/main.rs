#![allow(clippy::needless_update)]

use appy::components::blk::Dim::*;
use appy::components::{bg::*, blk::*, interaction::*, text::*, grid::*};
use appy::{apx, cb_with_clone, component, main_window};

use appy::core::element::*;
use appy::core::hooks::use_reducer;
use appy::utils::cb::Cb;

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

#[component]
pub struct TextButton {
	text: String,
	on_click: Cb
}

impl Element for TextButton {
	fn render(self:ElementWrap<Self>)->Elements {
		let hover_state=use_hover_state_ref();
		let c=match *hover_state {
			HoverState::Normal=>0x808080,
			HoverState::Active=>0x404040,
			HoverState::Hover=>0xc0c0c0,
		};

		apx! {
			<bg col=c/>
			<text text=self.text align=Align::Center size=Pc(60.0) col=0xffffff/>
			<interaction hover_state_ref=hover_state on_click=self.on_click/>
		}
	}
}

#[main_window]
pub fn app()->Elements {
	let state=use_reducer(AppState::reducer,||AppState{v:1});

	let s=format!("Value: {}",(*state).v);

	apx!{
		<blk top=Pc(25.0) height=Pc(25.0)>
			<text text=s.to_string() align=Align::Center size=Pc(60.0) col=0xffffff/>
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