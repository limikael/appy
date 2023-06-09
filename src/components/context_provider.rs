use crate::core::Appy;
use appy::hooks::use_post_render;
use appy::types::*;
use appy::*;
use std::any::TypeId;
use std::rc::Rc;

/// Provide context.
///
/// Provide a context to be used together with [`use_context`](appy::hooks::use_context()).
#[derive_component(ComponentBuilder, SnakeFactory)]
pub struct ContextProvider<T: 'static> {
    value: Option<Rc<T>>,
}

impl<T> Default for ContextProvider<T> {
    fn default() -> Self {
        Self {
            value: None,
            children: vec![],
            key: None,
        }
    }
}

#[function_component]
fn _context_provider<T>(props: ContextProvider<T>) -> Elements {
    let t = props.value.clone().unwrap();

    Appy::with(|appy| {
        let type_id = TypeId::of::<T>();

        if !appy.contexts.contains_key(&type_id) {
            appy.contexts.insert(type_id, vec![]);
        }

        appy.contexts.get_mut(&type_id).unwrap().push(t);
    });

    use_post_render(Box::new(|| {
        Appy::with(|appy| {
            let type_id = TypeId::of::<T>();

            appy.contexts.get_mut(&type_id).unwrap().pop();
        });
    }));

    props.children
}
