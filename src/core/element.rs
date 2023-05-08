use appy::{types::*, *};

pub fn flatten_elements(el: &mut [Elements]) -> Elements {
    let mut res: Elements = vec![];

    for sub in el {
        res.append(sub)
    }

    res
}

#[derive_component(ComponentBuilder, Default, SnakeFactory)]
pub struct RootElement {
    root: Option<fn() -> Elements>,
}

#[function_component]
fn _root_element(props: RootElement) -> Elements {
    (props.root.unwrap())()
}
