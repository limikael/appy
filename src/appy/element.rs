//use crate::{*};
//use std::rc::Rc;
//use std::any::Any;

pub trait ElementT {
    fn render(self: Box<Self>) -> Elements;
}

//#[derive(Debug)]
pub struct Element<T> {
    props: T,
    renderer: fn(T, Elements) -> Elements,
    children: Elements,
}

impl<T: 'static> Element<T> {
    pub fn call_render(self) -> Elements {
        (self.renderer)(self.props, self.children)
    }

    pub fn create(
        renderer: fn(T, Elements) -> Elements,
        props: T,
        children: Elements,
    ) -> Box<dyn ElementT> {
        Box::new(Self {
            renderer,
            props,
            children,
        })
    }
}

impl<T: 'static> ElementT for Element<T> {
    fn render(self: Box<Self>) -> Elements {
        self.call_render()
    }
}

pub type Elements = Vec<Box<dyn ElementT>>;

pub fn flatten_elements(el: &mut Vec<Elements>) -> Elements {
    let mut res: Elements = vec![];

    for sub in el {
        res.append(sub)
    }

    res
}
