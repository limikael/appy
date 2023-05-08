use appy::{components::*, hooks::*, types::*, *};
use std::rc::Rc;

#[derive_component(Default, ComponentBuilder, SnakeFactory)]
pub struct Button {
    text: String,
    on_click: Option<Rc<dyn Fn()>>,
}

#[function_component]
fn _button(props: Button) -> Elements {
    let hover_state = use_hover_state_ref();
    let c = match *hover_state {
        HoverState::Normal => 0x808080,
        HoverState::Hover => 0xc0c0c0,
        HoverState::Active => 0x404040,
    };

    apx! {
        <bg color=c/>
        <text text=&*props.text size=pct(50) align=Align::Center/>
        <interaction on_click=props.on_click.unwrap() hover_state_ref=hover_state/>
    }
}

#[main_window]
pub fn main() -> Elements {
    let x = use_spring(|| 0.0, SpringConf::DEFAULT);

    //println!("render");

    apx! {
        <Blk left=10 bottom=10 height=90 width=150>
            <Button text="smooth"
                    on_click=rc_with_clone!([x],move||x.target(0.0))/>
        </Blk>

        <Blk right=10 bottom=10 height=90 width=150>
            <Button text="smooth"
                    on_click=rc_with_clone!([x],move||x.target(100.0))/>
        </Blk>

        <Blk right=10 bottom=110 height=90 width=150>
            <Button text="chop"
                    on_click=rc_with_clone!([x],move||x.set(100.0))/>
        </Blk>

        <Blk right=50>
            <Blk left=pct(*x) width=50 height=50 top=50>
                <Bg color=0xff0000/>
            </Blk>
        </Blk>
    }
}
