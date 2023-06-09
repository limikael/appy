#![allow(clippy::needless_update)]

use appy::{components::*, hooks::*, types::*, *};
use std::rc::Rc;

#[derive(Clone)]
enum Action {
    Add,
    Sub,
}

#[derive(Clone)]
struct AppState {
    v: i32,
}

impl AppState {
    fn reducer(&self, a: Action) -> Self {
        match a {
            Action::Add => Self { v: self.v + 1 },
            Action::Sub => Self { v: self.v - 1 },
        }
    }
}

#[derive_component(Default, ComponentBuilder, SnakeFactory)]
pub struct TextButton {
    text: String,
    on_click: Option<Rc<dyn Fn()>>,
}

#[function_component]
fn _text_button(props: TextButton) -> Elements {
    let hover_state = use_hover_state_ref();
    let c = match *hover_state {
        HoverState::Normal => 0x808080,
        HoverState::Active => 0x404040,
        HoverState::Hover => 0xc0c0c0,
    };

    apx! {
        <bg color=c/>
        <text text=&*props.text align=Align::Center size=pct(60) color=0xffffff/>
        <interaction hover_state_ref=hover_state on_click=props.on_click.unwrap()/>
    }
}

#[main_window]
pub fn app() -> Elements {
    let state = use_reducer(AppState::reducer, || AppState { v: 1 });

    let s = format!("Value: {}", (*state).v);

    apx! {
        <blk top=pct(25) height=pct(25)>
            <text text=&*s align=Align::Center size=pct(60) color=0xffffff/>
        </blk>
        <grid cols=2>
            <blk top=pct(50) height=pct(20) width=pct(50)>
                <text_button text="-1"
                    on_click=rc_with_clone!([state],move||{
                        state.dispatch(Action::Sub);
                    })
                />
            </blk>
            <blk top=pct(50) height=pct(20) width=pct(50)>
                <text_button text="+1"
                    on_click=rc_with_clone!([state],move||{
                        state.dispatch(Action::Add);
                    })
                />
            </blk>
        </grid>
    }
}
