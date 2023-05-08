use appy::{components::*, hooks::*, types::*, *};
use glapp::AppEvent;

#[derive_component(ComponentBuilder, Default, SnakeFactory)]
pub struct Slider {
    value: Option<StateRef<f32>>,
    max: f32,
}

#[function_component]
fn _slider(props: Slider) -> Elements {
    let mut val = use_state(|| 0.0);
    if props.value.is_some() {
        val = props.value.unwrap();
    }

    let down = use_state(|| false);
    let app_context = use_context::<AppContext>();
    let max = props.max;

    use_app_event(rc_with_clone!([val, app_context, down, max], move |e| {
        let update = rc_with_clone!([val, app_context], move |x: f32| {
            let v = max * (x - app_context.rect.x) / app_context.rect.w;
            val.set(v.max(0.0).min(max));
        });

        match *e {
            AppEvent::MouseDown { x, y, .. } => {
                if app_context.rect.contains(x, y) {
                    update(x);
                    down.set(true);
                }
            }
            AppEvent::MouseMove { x, .. } => {
                if *down {
                    update(x);
                }
            }
            AppEvent::MouseUp { .. } => {
                down.set(false);
            }
            _ => {}
        }
    }));

    let p = (*val / max) * (app_context.rect.w - 20.);

    apx! {
        <blk height=20>
            <blk width=pct(100) height=10>
                <bg color=0x808080 corner_radius=5 border_width=1 border_color=0xffffff/>
            </blk>
            <blk width=20 height=20 left=p>
                <bg color=0xc0c0c0 border_width=1 border_color=0xffffff corner_radius=10/>
            </blk>
        </blk>
    }
}

#[derive_component(ComponentBuilder, Default, SnakeFactory)]
pub struct FlowItem {
    text: String,
}

#[function_component]
fn _flow_item(props: FlowItem) -> Elements {
    let app_context = use_context::<AppContext>();
    let w = app_context.default_font.get_str_width(&*props.text, 24.);

    apx! {
        <flow width=w+16. height=40>
            <bg color=0x808080 border_color=0xffffff border_width=1/>
            <text text=&*props.text size=24/>
        </flow>
    }
}

#[derive_component(ComponentBuilder, Default, SnakeFactory)]
pub struct Setting {
    text: String,
    value: Option<StateRef<f32>>,
    max: f32,
}

#[function_component]
fn _setting(props: Setting) -> Elements {
    apx! {
        <flow height=20>
            <text text=&*props.text/>
        </flow>
        <flow height=20>
            <slider max=props.max value_option=props.value/>
        </flow>
        <flow height=10/>
    }
}

#[main_window]
pub fn app() -> Elements {
    let width_pct = use_state(|| 50.0);
    let height_pct = use_state(|| 50.0);
    let num_words = use_state(|| 10.0);
    let gap = use_state(|| 10.0);
    let vgap = use_state(|| 10.0);
    let align = use_state(|| Align::Left);
    let valign = use_state(|| VAlign::Top);

    let lorem="Lorem ipsum dolor sit amet, consectetur adipiscing elit.
Aliquam convallis nisl sit amet neque iaculis commodo. Maecenas viverra imperdiet sem vel scelerisque.
Nullam cursus, justo id dapibus sollicitudin, nisl justo tempor massa, quis mollis dolor risus vitae lorem.
Nam id nunc dapibus nunc elementum congue. Sed augue justo, vehicula eu enim nec, maximus varius nisi.
Vivamus dapibus magna id nisl eleifend egestas. Cras rhoncus eros eu magna venenatis vestibulum.
Cras nisi sapien, finibus non risus eget, blandit vehicula mi. Fusce venenatis eleifend nisi, eu varius sem bibendum vitae.
Sed lobortis arcu ut tortor elementum, et luctus nunc malesuada. Etiam vestibulum.";

    let mut flow_items: Elements = vec![];
    let mut n: i32 = 0;
    for word in lorem.split([' ', '\n']) {
        if n <= *num_words as i32 {
            flow_items.append(&mut apx! {
                <flow_item text=word/>
            });
        }
        n += 1;
    }

    apx! {
        <blk left=0 width=200>
            <bg color=0x000080 border_color=0xffffff border_width=1 borders=[false,true,false,false]/>
            <blk margin=10 flow_valign=VAlign::Top>
                <setting
                    text=&*format!("Words: {:.0}",*num_words)
                    max=50.
                    value=num_words.clone()/>

                <setting
                    text=&*format!("Width: {:.0}%",*width_pct)
                    max=100.
                    value=width_pct.clone()/>

                <setting
                    text=&*format!("Height: {:.0}%",*height_pct)
                    max=100.
                    value=height_pct.clone()/>

                <setting
                    text=&*format!("Gap: {:.0}",*gap)
                    max=20.
                    value=gap.clone()/>

                <setting
                    text=&*format!("VGap: {:.0}",*vgap)
                    max=20.
                    value=vgap.clone()/>

                <flow height=20>
                    <text size=20 text="Align:"/>
                </flow>
                <flow height=20>
                    <grid cols=3>
                        <blk>
                            <bg color=0x808080 border_width=1 border_color=0xffffff/>
                            <text text="L"/>
                            <interaction on_click=rc_with_clone!([align],move||align.set(Align::Left))/>
                        </blk>
                        <blk>
                            <bg color=0x808080 border_width=1 border_color=0xffffff/>
                            <text text="C"/>
                            <interaction on_click=rc_with_clone!([align],move||align.set(Align::Center))/>
                        </blk>
                        <blk>
                            <bg color=0x808080 border_width=1 border_color=0xffffff/>
                            <text text="R"/>
                            <interaction on_click=rc_with_clone!([align],move||align.set(Align::Right))/>
                        </blk>
                    </grid>
                </flow>
                <flow height=20/>

                <flow height=20>
                    <text size=20 text="VAlign:"/>
                </flow>
                <flow height=20>
                    <grid cols=3>
                        <blk>
                            <bg color=0x808080 border_width=1 border_color=0xffffff/>
                            <text text="T"/>
                            <interaction on_click=rc_with_clone!([valign],move||valign.set(VAlign::Top))/>
                        </blk>
                        <blk>
                            <bg color=0x808080 border_width=1 border_color=0xffffff/>
                            <text text="M"/>
                            <interaction on_click=rc_with_clone!([valign],move||valign.set(VAlign::Middle))/>
                        </blk>
                        <blk>
                            <bg color=0x808080 border_width=1 border_color=0xffffff/>
                            <text text="B"/>
                            <interaction on_click=rc_with_clone!([valign],move||valign.set(VAlign::Bottom))/>
                        </blk>
                    </grid>
                </flow>
                <flow height=20/>
            </blk>
        </blk>
        <blk left=200 right=0 height=pct(100)>
            <blk width=pct(*width_pct) height=pct(*height_pct)
                flow_gap=*gap flow_vgap=*vgap
                flow_align=*align flow_valign=*valign>
                <bg color=0x000080/>
                {flow_items}
            </blk>
        </blk>
    }
}
