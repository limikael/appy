use appy::{components::*, hooks::*, types::*, *};
use rand::{distributions::Alphanumeric, Rng};
use std::rc::Rc;

#[derive_component(ComponentBuilder, Default, SnakeFactory)]
pub struct FlowButton {
    text: String,
    on_click: Option<Rc<dyn Fn()>>,
}

#[function_component]
fn _flow_button(p: FlowButton) -> Elements {
    let hover_state = use_hover_state_ref();
    let app_context = use_context::<AppContext>();
    let w = app_context.default_font.get_str_width(&*p.text, 24.0);

    let c = match *hover_state {
        HoverState::Normal => 0x808080,
        HoverState::Active => 0x404040,
        HoverState::Hover => 0xa0a0a0,
    };

    apx! {
        <flow width=w+16.0 height=48>
            <blk top=8 bottom=8>
                <bg color=c/>
                <text size=24 text=&*p.text/>
            </blk>
            <interaction hover_state_ref=hover_state on_click_option=p.on_click/>
        </flow>
        <flow width=8/>
    }
}

#[derive_component(ComponentBuilder, Default, SnakeFactory)]
pub struct ListItem {
    text: String,
}

#[function_component]
fn _list_item(p: ListItem) -> Elements {
    let selected = use_state(|| false);

    apx! {
        <bg color=match *selected {true=>0x0000ff, false=>0xffffff}/>
        <text size=pct(100) text=&*p.text
            color=match *selected {true=>0xffffff, false=>0x000000}/>
        <interaction on_click=rc_with_clone!([selected],move||selected.set(!*selected))/>
    }
}

#[derive(Clone)]
enum AppAction {
    AddStart,
    AddEnd,
    RemoveStart,
    RemoveEnd,
}

#[derive(Clone)]
struct AppState {
    items: Vec<String>,
}

fn random_string() -> String {
    let num = rand::thread_rng().gen_range(8..16);

    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(num)
        .map(char::from)
        .collect()
}

impl AppState {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn action(&self, action: AppAction) -> Self {
        let mut new_self = self.clone();
        new_self.action_mut(action);
        new_self
    }

    pub fn action_mut(&mut self, action: AppAction) {
        match action {
            AppAction::AddStart => {
                self.items.insert(0, random_string());
            }

            AppAction::RemoveStart => {
                if self.items.len() > 0 {
                    self.items.remove(0);
                }
            }

            AppAction::AddEnd => {
                self.items.push(random_string());
            }

            AppAction::RemoveEnd => {
                if self.items.len() > 0 {
                    self.items.remove(self.items.len() - 1);
                }
            }
        }
    }
}

#[main_window]
fn app() -> Elements {
    let app = use_reducer(AppState::action, AppState::new);

    apx! {
        <blk top=0 height=48>
            <bg color=0x000000/>
            <flow width=8/>
            <flow_button text="+ Start"
                on_click=rc_with_clone!([app],move||app.dispatch(AppAction::AddStart))/>
            <flow_button text="- Start"
                on_click=rc_with_clone!([app],move||app.dispatch(AppAction::RemoveStart))/>
            <flow_button text="+ End"
                on_click=rc_with_clone!([app],move||app.dispatch(AppAction::AddEnd))/>
            <flow_button text="- End"
                on_click=rc_with_clone!([app],move||app.dispatch(AppAction::RemoveEnd))/>
        </blk>
        <blk top=48>
            <bg color=0x000080/>
            {app.items.iter().flat_map(|item|{
                apx!{
                    <flow height=50 key=item.clone()>
                        <blk margin=5>
                            <list_item text=&*item/>
                        </blk>
                    </flow>
                }
            }).collect()}
        </blk>
    }
}
