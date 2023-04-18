use appy::{derive_component,ComponentBuilder,SnakeFactory};
use crate::types::{Element, Elements, ElementWrap};

pub fn flatten_elements(el: &mut [Elements]) -> Elements {
    let mut res: Elements = vec![];

    for sub in el {
        res.append(sub)
    }

    res
}

#[derive_component(ComponentBuilder,Default,SnakeFactory)]
pub struct RootElement {
    root: Option<fn()->Elements>
}

impl Element for RootElement {
    fn render(self:ElementWrap<Self>)->Elements {
        (self.root.unwrap())()
    }
}
