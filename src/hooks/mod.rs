use crate::core::component::HookRef;
use crate::core::Appy;
use crate::types::*;
use glapp::AppEvent;
use std::any::TypeId;
use std::rc::Rc;

mod use_reducer;
pub use use_reducer::*;

mod use_spring;
pub use use_spring::*;

pub type StateRef<T> = HookRef<T>;

/// Track state in a function component.
pub fn use_state<F, T: 'static>(ctor: F) -> StateRef<T>
where
    F: Fn() -> T,
{
    Appy::with(|appy| appy.use_hook_ref(ctor))
}

/// Post render handler.
///
/// The function specified will be called after the children of the
/// current component has been rendered.
pub fn use_post_render(f: Box<dyn FnOnce()>) {
    Appy::with(|appy| {
        appy.with_current_component_instance(|ci| {
            ci.post_render = Some(f);
        })
    })
}

/// Second render pass.
///
/// The function specified will be called after the children of the
/// current component has been rendered.
pub fn use_second_render_pass(f: Box<dyn FnOnce() -> Elements>) {
    Appy::with(|appy| {
        appy.with_current_component_instance(|ci| {
            ci.second_render = Some(f);
        })
    })
}

/// Low level event handler.
///
/// Function handler for low level application events.
pub fn use_app_event(f: Rc<dyn Fn(&AppEvent)>) {
    Appy::with(|appy| {
        appy.app_event_handlers.push(f.clone());
    })
}

/// Animation frame.
///
/// The registered function will be called after the next render.
pub fn use_animation_frame(f: Rc<dyn Fn(f32)>) {
    Appy::with(|appy| {
        appy.animation_frame_handlers.push(f.clone());
    })
}

/// A context is a way to access global state.
///
/// A context can be provided with [`ContextProvider`](appy::components::ContextProvider),
/// and then accessed further down in the tree with this function.
pub fn use_context<T: 'static>() -> Rc<T> {
    let type_id = TypeId::of::<T>();

    Appy::with(|appy| {
        let v = appy.contexts.get(&type_id).unwrap();
        let any = v[v.len() - 1].clone();
        any.downcast::<T>().unwrap()
    })
}

/// Used to continously check the hover state of an interaction component.
///
/// For example:
/// ```
/// use appy::{*, components::*, types::*, hooks::*};
///
/// #[main_window]
/// pub fn app()->Elements {
///     let hover_state=use_hover_state_ref();
///     let color=match *hover_state {
///         HoverState::Normal=>0x808080,
///         HoverState::Active=>0x404040,
///         HoverState::Hover=>0xc0c0c0,
///     };
///
///     apx!{
///         <blk top=pct(25) height=pct(25) width=pct(50) height=pct(50)>
///             <bg color=color/>
///             <interaction hover_state_ref=hover_state/>
///         </blk>
///     }
/// }
/// ```
pub fn use_hover_state_ref() -> StateRef<HoverState> {
    use_state(|| HoverState::Normal)
}

/// Get a Font from data.
///
/// A font is a rather heavy resource, the best way to use it is probably
/// to create it once and pass it down using a context or prop drilling.
///
/// Example:
/// ```
/// use appy::{*, hooks::*, components::*, types::*};
///
/// #[main_window]
/// pub fn main()->Elements {
///	    let font=use_font_data(||include_bytes!("../core/Roboto-Regular.ttf"));
///
///	    apx!{
///		    <Text text="Hello World" font=font size=100/>
///	    }
/// }
/// ```
pub fn use_font_data<F>(closure: F) -> Rc<Font>
where
    F: Fn() -> &'static [u8],
{
    let state_ref = use_state(|| Font::from_data(closure()));

    state_ref.as_rc()
}

/// Get an ImageSource from data.
pub fn use_image_data<F>(closure: F)->Rc<ImageSource>
where
    F: Fn() -> &'static [u8],
{
    let state_ref = use_state(|| ImageSource::from_memory(closure()));

    state_ref.as_rc()
}