#![allow(clippy::needless_update)]

use appy::{components::*, hooks::*, types::*, *};

#[main_window]
pub fn app() -> Elements {
    let v = use_state(|| 1);
    let hover_state = use_hover_state_ref();

    let on_button_click = rc_with_clone!([v], move || {
        //println!("set: {}",*v);

        v.set(*v + 1);
    });

    let s = format!("Hello: {}", *v);
    let c = match *hover_state {
        HoverState::Normal => 0x808080,
        HoverState::Active => 0x404040,
        HoverState::Hover => 0xc0c0c0,
    };

    apx! {
        <blk top=pct(25) height=pct(25)>
            <text text=&*s align=Align::Center size=pct(60) color=0xffffff/>
        </blk>
        <blk top=pct(50) height=pct(20) width=pct(25)>
            <bg color=c border_width=1 border_color=0xffffff corner_radius=10/>
            <text text="+1" align=Align::Center size=pct(60) color=0xffffff/>
            <interaction hover_state_ref=hover_state on_click=on_button_click/>
        </blk>
    }
}
