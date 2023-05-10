use appy::{components::*, hooks::*, types::*, *};
use std::rc::Rc;

mod calculator_model;
use calculator_model::CalculatorModel;

#[derive_component(Default, ComponentBuilder, SnakeFactory)]
pub struct ButtonBg {
    pub on_click: Option<Rc<dyn Fn()>>,
    pub no_hover: bool
}

#[function_component]
fn _button_bg(props: ButtonBg) -> Elements {
    let hover_state = use_hover_state_ref();
    //println!("state: {:?}",*hover_state);

    let (color,alpha) = match *hover_state {
        HoverState::Normal => (0x000000,0.0),
        HoverState::Hover => (0xffffff,if props.no_hover {0.0} else {0.25}),
        HoverState::Active => (0x000000,0.25),
    };

    apx! {
        <blk alpha=alpha>
            <bg color=color/>
            <interaction on_click_option=props.on_click hover_state_ref=hover_state/>
        </blk>
    }
}

#[derive_component(Default, ComponentBuilder, SnakeFactory)]
pub struct Button {
    on_click: Option<Rc<dyn Fn(char)>>,
    id: char,
    color: u32
}

#[function_component]
fn _button(props: Button) -> Elements {
    let self_on_click = props.on_click.as_ref().unwrap().clone();
    let self_id = props.id.clone();
    let on_click = rc_with_clone!([], move || { (self_on_click)(self_id) });

    apx!(
        <blk left=pct(10) top=pct(10) right=pct(10) bottom=pct(10)>
            <bg color=props.color corner_radius=5/>
            <text text=&*props.id.to_string() size=pct(65) align=Align::Center color=0x000000/>
            <button_bg on_click=on_click/>
        </blk>
    )
}

#[derive_component(Default, ComponentBuilder, SnakeFactory)]
pub struct TopBar {
    on_click: Option<Rc<dyn Fn()>>,
    color: u32
}

#[function_component]
fn _top_bar(props: TopBar) -> Elements {
    let hamburger = use_image_data(|| include_bytes!("assets/hamburger-icon.png"));
    apx! {
        <blk top=0 height=56>
            <bg color=props.color/>
            <blk left=0 width=56>
                <blk left=8 height=39.9 width=39.9>
                    <img src=hamburger/>
                </blk>
                <button_bg on_click_option=props.on_click no_hover=true/>
            </blk>
            <text size=pct(50) text="Appy Calculator"/>
        </blk>
    }
}

#[derive_component(Default, ComponentBuilder, SnakeFactory)]
pub struct ColorSchemeSelect {
    scheme: ColorScheme,
    selected: bool,
    on_click: Option<Rc<dyn Fn()>>
}

#[function_component]
fn _color_scheme_select(props:ColorSchemeSelect)->Elements {
    let hover_state=use_hover_state_ref();

    let selected=props.selected||match *hover_state {
        HoverState::Normal=>false,
        HoverState::Hover=>true,
        HoverState::Active=>true,
    };

    apx!{
        <blk margin=5>
            <bg corner_radius=5
                    border_color=if selected {0x808080} else {0xc0c0c0}
                    color=if selected {0xe0e0e0} else {0xf0f0f0}  
                    border_width=if selected {2} else {1}
            />
            <blk margin=10>
                <grid rows=1 cols=3>
                    <bg color=props.scheme.background />
                    <bg color=props.scheme.display />
                    <bg color=props.scheme.buttons />
                </grid>
            </blk>
            <interaction on_click_option=props.on_click
                    hover_state_ref=hover_state/>
        </blk>
    }
}

#[derive_component(Default, ComponentBuilder, SnakeFactory)]
pub struct Menu {
    show: bool,
    on_close: Option<Rc<dyn Fn()>>,
    color_schemes: Vec<ColorScheme>,
    color_scheme_index: usize,
    on_select_index: Option<Rc<dyn Fn(usize)>>,
    on_about_click: Option<Rc<dyn Fn()>>,
}

#[function_component]
fn _menu(props: Menu) -> Elements {
    let logo = use_image_data(|| include_bytes!("assets/menu-logo.png"));
    let info =use_image_data(|| include_bytes!("assets/circle-info-solid.png"));
    let xmark =use_image_data(|| include_bytes!("assets/circle-xmark-solid.png"));
    let quit=use_quit_trigger();

    let left = use_spring(|| -300.0, SpringConf::spring(350.0, 30.0).epsilon(1.0));
    let alpha = use_spring(|| 0.0, SpringConf::linear(2.0));

    if props.show {
        left.target(0.0);
        alpha.target(0.5);
    }

    else {
        left.target(-300.0);
        alpha.target(0.0);
    }

    if *alpha==0.0 && *left==-300.0 {
        return apx!{};
    }

    let on_select_index=props.on_select_index.as_ref().unwrap().clone();

    apx! {
        <blk>
            <blk alpha=*alpha>
                <bg color=0x000000 />
            </blk>
            <interaction on_click_option=props.on_close/>
            <blk left=*left width=300>
                <interaction/>
                <bg color=0xf0f0f0/>
                <img src=logo valign=VAlign::Top/>
                <blk top=150 height=0>
                    <bg border_width=1
                            border_color=0xc0c0c0 
                            borders=[true,false,false,false]/>
                </blk>
                <blk top=160 height=150 left=5 right=5>
                    <grid cols=3 rows=2>
                        {props.color_schemes.iter().enumerate().map(|(i,c)| {
                            apx!{
                                <color_scheme_select
                                        selected=i==props.color_scheme_index
                                        scheme=c.clone()
                                        on_click=rc_with_clone!([i,on_select_index],move||{
                                            on_select_index(i);
                                        })
                                />
                            }
                        }).flatten().collect()}
                    </grid>
                </blk>
                <blk bottom=0 height=90>
                    <grid rows=1 cols=2>
                        <blk>
                            <blk height=40 top=10>
                                <img src=xmark/>
                            </blk>
                            <blk height=20 bottom=10>
                                <text size=pct(100) color=0xa0a0a0 text="Quit"/>
                            </blk>
                            <button_bg on_click=quit/>
                        </blk>
                        <blk>
                            <blk height=40 top=10>
                                <img src=info/>
                            </blk>
                            <blk height=20 bottom=10>
                                <text size=pct(100) color=0xa0a0a0 text="About"/>
                            </blk>
                            <button_bg on_click_option=props.on_about_click/>
                        </blk>
                    </grid>
                </blk>
            </blk>
        </blk>
    }
}

#[derive_component(ComponentBuilder, SnakeFactory, Default)]
pub struct About {
    show: Option<StateRef<bool>>
}

#[function_component]
pub fn _about(props:About)->Elements {
    let bg_alpha = use_spring(|| 0.0, SpringConf::linear(2.0));
    let top = use_spring(|| 100.0, SpringConf::spring(250.0, 30.0).epsilon(1.0));
    let alpha = use_spring(|| 0.0, SpringConf::linear(2.0));
    let app_context=use_context::<AppContext>();

    let texts=vec![
        "Appy Calculator",
        "",
        "This is a demonstration of",
        "the Appy declarative UI framework.",
        "",
        "The app is written in pure Rust,",
        "rendered natively without the use of a",
        "web view or similar.",
    ];

    let show=props.show.as_ref().unwrap().clone();
    if *show {
        top.target(0.0);
        bg_alpha.target(0.5);
        alpha.target(1.0);
    }

    else {
        top.target(100.0);
        bg_alpha.target(0.0);
        alpha.target(0.0);
    }

    if *alpha==0.0 && *bg_alpha==0.0 && *top==100.0 {
        return apx!{}
    }

    apx!{
        <interaction
                on_click=rc_with_clone!([show],move||{
                    show.set(false);
                })
        />
        <blk alpha=*bg_alpha>
            <bg color=0x000000/>
        </blk>
        <blk alpha=*alpha top=*top height=app_context.rect.h>
            <blk left=30 right=30 height=350>
                <interaction/>
                <bg color=0xffffff corner_radius=20 border_color=0xc0c0c0 border_width=1/>
                <blk top=0 height=60>
                    <text size=pct(50) text="Appy Calculator" color=0x000000/>
                </blk>

                <blk top=60 bottom=60>
                    <bg color=0xf0f0f0 border_color=0xc0c0c0 border_width=1/>
                </blk>
                <blk bottom=10 height=40 width=100>
                    <bg color=0x2F8FD3 corner_radius=5/>
                    <text size=pct(50) text="Close" color=0xffffff/>
                    <button_bg 
                            on_click=rc_with_clone!([],move||{
                                show.set(false);
                            })
                    />
                </blk>

                {texts.iter().map(|s|{
                    apx!{
                        <flow height=20>
                            <text text=s size=pct(100) color=0x000000/>
                        </flow>
                    }
                }).flatten().collect()}
            </blk>
        </blk>
    }
}

#[derive_component(ComponentBuilder,Default,SnakeFactory)]
pub struct Display {
    text: String
}

#[function_component]
fn _display(props:Display)->Elements {
    let app_context=use_context::<AppContext>();
    let mut size=app_context.rect.h*0.5;

    let v=app_context.default_font.get_str_width(&*props.text,size);
    if v>app_context.rect.w {
        size*=app_context.rect.w/v
    }

    apx!{
        <text align=Align::Right
                text=&*props.text
                size=size
                color=0xffffff/>
    }
}

#[derive(Default,Clone)]
pub struct ColorScheme {
    background: u32,
    buttons: u32,
    display: u32
}

#[main_window]
fn app() -> Elements {
    let model = use_reducer(CalculatorModel::action, CalculatorModel::new);
    let show_info = use_state(|| false);
    let show_about=use_state(||false);
    let color_scheme_index=use_state(||0usize);
    let color_schemes=vec![
        ColorScheme{background:0x69140E,display:0x3C1518,buttons:0xD58936},
        ColorScheme{background:0x32007F,display:0x1B0050,buttons:0xDE8EFD},
        ColorScheme{background:0x59981A,display:0x3D550C,buttons:0xECF87F}
    ];

    let color_scheme=&color_schemes[*color_scheme_index];

    apx!(
        <top_bar color=color_scheme.background
                on_click=rc_with_clone!([show_info],move||{
                    show_info.set(!*show_info)
                })
        />
        <blk top=56 left=0 right=0 bottom=0>
            <blk height=pct(25) top=0>
                <bg color=color_scheme.display/>
                <blk left=pct(5) right=pct(5)>
                    <display text=&*model.get_display_value()/>
                </blk>
            </blk>
            <blk top=pct(25)>
                <bg color=color_scheme.background/>
                <blk margin=10>
                    <grid rows=5 cols=4>
                        {"C«%/789*456-123+±0.=".chars().into_iter().flat_map(|c| {
                            apx!{
                                <button id=c color=color_scheme.buttons
                                        on_click=rc_with_clone!([model],move|c|{
                                            model.dispatch(c)
                                        })
                                />
                            }
                        }).collect()}
                    </grid>
                </blk>
            </blk>
        </blk>
        <menu show=*show_info 
                color_schemes=color_schemes
                color_scheme_index=*color_scheme_index
                on_close=rc_with_clone!([show_info],move||{
                    if *show_info {
                        show_info.set(false)
                    }
                })
                on_select_index=rc_with_clone!([show_info],move|i|{
                    color_scheme_index.set(i);
                    show_info.set(false);
                })
                on_about_click=rc_with_clone!([show_about,show_info],move||{
                    show_about.set(true);
                    show_info.set(false);
                })
        />
        <about show=show_about.clone()/>
    )
}
