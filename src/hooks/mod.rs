use std::rc::Rc;
use std::any::TypeId;
use glapp::{AppEvent};
use crate::core::Appy;
use crate::core::component::HookRef;
use crate::{types::*};

mod use_reducer;
pub use use_reducer::*;

mod use_spring;
pub use use_spring::*;

pub type StateRef<T>=HookRef<T>;

/// Track state in a function component.
pub fn use_state<F, T: 'static>(ctor: F)->StateRef<T>
		where F:Fn()->T {
	Appy::with(|appy|{
		appy.use_hook_ref(ctor)
	})
}

/// Post render handler.
///
/// The function specified will be called after the children of the
/// current component has been rendered.
pub fn use_post_render(f: Rc<dyn Fn()>) {
	Appy::with(|appy|{
		appy.with_current_component_instance(|ci|{
			ci.post_render=Some(f.clone());
		})
	})
}

/// Low level event handler.
///
/// Function handler for low level application events.
pub fn use_app_event(f: Rc<dyn Fn(&AppEvent)>) {
	Appy::with(|appy|{
		appy.app_event_handlers.push(f.clone());
	})
}

/// Animation frame.
///
/// The registered function will be called after the next render.
pub fn use_animation_frame(f: Rc<dyn Fn(f32)>) {
	Appy::with(|appy|{
		appy.animation_frame_handlers.push(f.clone());
	})
}

/// A context is a way to access global state.
///
/// A context can be provided with [`ContextProvider`](appy::components::ContextProvider),
/// and then accessed further down in the tree with this function.
pub fn use_context<T: 'static>()->Rc<T> {
	let type_id=TypeId::of::<T>();

	Appy::with(|appy|{
		let v=appy.contexts.get(&type_id).unwrap();
		let any=v[v.len()-1].clone();
		any.downcast::<T>().unwrap()
	})
}

/// Used to continously check the hover state of an interaction component.
///
/// For example:
/// ```
/// #[main_window]
/// pub fn app()->Elements {
///     let hover_state=use_hover_state_ref();
///     let col=match *hover_state {
///         HoverState::Normal=>0x808080,
///         HoverState::Active=>0x404040,
///         HoverState::Hover=>0xc0c0c0,
///     };
///
///     apx!{
///         <blk top=Pc(25.0) height=Pc(25.0) width=Pc(50.0) height=Pc(50.0)>
///             <bg col=col/>
///             <interaction hover_state_ref=Some(hover_state)/>
///         </blk>
///     }
/// }
/// ```
pub fn use_hover_state_ref()->StateRef<HoverState> {
    use_state(||HoverState::Normal)
}

/// Get a font face from data.
///
/// In order to draw text, use:
///
/// - [`use_font_face`](use_font_face) - To get the data for the font.
/// - [`use_font`](use_font) - To render the font to a texture for a specific size.
/// - [`Text`](crate::components::Text) - To render text on screen.
///
/// Example:
/// ```
///	let font_face=use_font_face(||include_bytes!("./Roboto-Regular.ttf"));
///	let font=use_font(font_face,100.0);
///
///	apx!{
///		<Text text="Hello World" font=font/>
///	}
/// ```
pub fn use_font_data<F>(closure: F)->Rc<Font> 
		where F: Fn()->&'static [u8] {
	let state_ref=use_state(||{
		Font::from_data(closure())
	});

	state_ref.as_rc()
}
