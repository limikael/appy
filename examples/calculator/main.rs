use appy::{components::*, hooks::*, types::*, *};
use std::rc::Rc;

mod calculator_model;
use calculator_model::CalculatorModel;

#[derive_component(Default, ComponentBuilder, SnakeFactory)]
pub struct Button {
    on_click: Option<Rc<dyn Fn(char)>>,
    id: char,
}

#[function_component]
fn _button(props: Button) -> Elements {
    let hover_state_ref = use_hover_state_ref();
    let self_on_click = props.on_click.as_ref().unwrap().clone();
    let self_id = props.id.clone();
    let on_click = rc_with_clone!([], move || { (self_on_click)(self_id) });

    let color = match *hover_state_ref {
        HoverState::Normal => 0xD58936,
        HoverState::Hover => 0xDEA260,
        HoverState::Active => 0xB36F25,
    };

    apx!(
        <blk left=pct(10) top=pct(10) right=pct(10) bottom=pct(10)>
            <interaction on_click=on_click hover_state_ref=hover_state_ref/>
            <bg color=color corner_radius=5/>
            <text text=&*props.id.to_string() size=pct(65) align=Align::Center color=0x000000/>
        </blk>
    )
}

#[derive_component(Default, ComponentBuilder, SnakeFactory)]
pub struct ButtonBg {
    pub on_click: Option<Rc<dyn Fn()>>,
    pub normal: u32,
    pub active: u32,
    pub hover: u32,
}

#[function_component]
fn _button_bg(props: ButtonBg) -> Elements {
    let hover_state = use_hover_state_ref();
    //println!("state: {:?}",*hover_state);

    let c = match *hover_state {
        HoverState::Normal => props.normal,
        HoverState::Hover => props.hover,
        HoverState::Active => props.active,
    };

    apx! {
        <bg color=c/>
        <interaction on_click=props.on_click.unwrap() hover_state_ref=hover_state/>
    }
}

#[derive_component(Default, ComponentBuilder, SnakeFactory)]
pub struct TopBar {
    on_click: Option<Rc<dyn Fn()>>
}

#[function_component]
fn _top_bar(props:TopBar)->Elements {
    let hamburger=use_image_data(||include_bytes!("hamburger-icon.png"));
    apx!{
        <blk top=0 height=56>
            <bg color=0x69140E/>
            <blk left=5 height=46 width=46>
                <img src=hamburger/>
                <interaction on_click_option=props.on_click/>
            </blk>
            <text size=pct(50) text="Appy Calculator"/>
        </blk>
    }
}

#[derive_component(Default, ComponentBuilder, SnakeFactory)]
pub struct MobileMenu {
    show: bool,
    on_close: Option<Rc<dyn Fn()>>
}

#[function_component]
fn _mobile_menu(props:MobileMenu)->Elements {
    let left = use_spring(||-300.0, SpringConf::DEFAULT);
    use_state(||{
        left.target(0.0);
    });

    //println!("l: {:?}",*left);

    if props.show {
        apx!{
            <blk>
                <bg color=0x000000 alpha=0.5/>
                <interaction on_click_option=props.on_close/>

                <blk left=*left width=300>
                    <bg color=0xffffff/>
                </blk>
            </blk>
        }
    }

    else {
        apx!{}
    }
}

#[main_window]
fn app() -> Elements {
    let model = use_reducer(CalculatorModel::action, CalculatorModel::new);
    let show_info = use_state(|| false);

    let on_click = rc_with_clone!([model], move |c: char| {
        model.dispatch(c);
    });

    let on_info_click = rc_with_clone!([show_info], move || {
        show_info.set(!*show_info);
    });

    apx!(
        <top_bar on_click=rc_with_clone!([show_info],move||{
            show_info.set(!*show_info)
        })/>
        <blk top=56 left=0 right=0 bottom=0>
            <blk height=pct(25) top=0>
                <bg color=0x3C1518/>
                <blk left=pct(5) right=pct(5)>
                    <text align=Align::Right
                            text=&*model.get_display_value() size=pct(50) color=0xffffff/>
                </blk>
            </blk>
            <blk top=pct(25)>
                <bg color=0x69140E/>
                <blk margin=10>
                    <grid rows=5 cols=4>
                        {"C«%/789*456-123+±0.=".chars().into_iter().flat_map(|c| {
                            apx!{
                                <button id=c on_click=on_click.clone() />
                            }
                        }).collect()}
                    </grid>
                </blk>
            </blk>
        </blk>
        {if *show_info {
            apx!{<mobile_menu show=*show_info on_close=rc_with_clone!([show_info],move||{
                show_info.set(false)
            })/>}
        } else {apx!{}}}
    )
}
