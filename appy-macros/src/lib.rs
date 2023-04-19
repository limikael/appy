//! # Appy - Declarative UI Framework for Native Application
//!
//! This crate contains macros for [Appy](https://github.com/limikael/appy).
//!
//! It is published as a separate crate  due to technical reasons. See
//! this [stack overflow thread](https://stackoverflow.com/questions/56713877/why-do-proc-macros-have-to-be-defined-in-proc-macro-crate).
//!
//! If it is not the case any more that it needs to be separate, please let me know!
use proc_macro::{*};

mod function_component;
#[proc_macro_attribute]
pub fn function_component(attr: TokenStream, input: TokenStream) -> TokenStream {
	function_component::function_component(attr,input)
}

mod main_window;
#[proc_macro_attribute]
/// Application entry point.
///
/// The main window macro defines the main application entry point.
/// It should be placed on a function that returns Elements. In this
/// sense, it is similar to the render function of an Element, but it
/// does not take any properties or children.
///
/// # Example
/// ```rust
/// use appy::*;
///
/// #[main_window]
/// pub fn app()->Elements {
///    apx!{
///        <bg col=0x800000/>
///        <text text="Hello World".to_string() align=Align::Center/>
///    }
/// }
/// ```
pub fn main_window(attr: TokenStream, input: TokenStream) -> TokenStream {
	main_window::main_window(attr,input)
}

mod derive_component;
/// Create a component.
///
/// A component takes input in the form of props and children,
/// and return elements describing what should appear on the screen.
///
/// # Example
/// ```rust
/// #[derive_component(Default,ComponentBuilder)]
/// pub struct MyComp {
///   param1: i32,
///   param2: i32
/// }
///
/// impl Element for MyComp {
///   pub fn render(self: ElementWrap<Self>)->Elements {
///     apx! {
///       <bg col=0x000000/>
///     }
///   }
/// }
/// ```
///
/// The component declared above can be used together with the
/// apx macro:
/// ```rust
/// apx!{
///   <MyComp param1=123 param2=456/>
///	}
/// ```
#[proc_macro_attribute]
pub fn derive_component(attr: TokenStream, input: TokenStream) -> TokenStream {
	derive_component::derive_component(attr,input)
}

mod component_builder;
/// Create builder.
///
/// Create a builder implementation. For example, when placed on the following struct:
/// ```rust
/// struct MyStruct {
///     an_int:i32,
///     an_optional_string:Option<String>
/// }
/// ```
/// This macro will create the following implementation:
/// ```rust
/// impl MyStruct {
///     pub fn new()->ElementWrap<Self> {
///         // ...
///     }
///
///     pub fn an_int(ElementWrap<Self>,v:i32)->ElementWrap<Self> {
///         // ...
///     }
///
///     pub fn an_optional_string(ElementWrap<Self>,v:String)->ElementWrap<Self> {
///         // ...
///     }
///	}
/// ```
/// Notes:
/// - For optional properties, e.g. fields wrapped in `Option<...>`, the setter in
///   the builder will take the wrapped type as parameter and call `Some()` with the
///   value.
#[proc_macro_derive(ComponentBuilder)]
pub fn component_builder(input: TokenStream) -> TokenStream {
	component_builder::component_builder(input)
}

mod snake_factory;
/// Produce snake cased factory function.
///
/// When placed on a `struct`, e.g. `MyStruct`, this derive macro
/// will create a function `my_struct` defined as:
/// ```rust
/// fn my_struct() -> MyStruct {
///     MyStruct::new()
/// }
/// ```
#[proc_macro_derive(SnakeFactory)]
pub fn snake_factory(input: TokenStream) -> TokenStream {
	snake_factory::snake_factory(input)
}

mod apx;
/// Process element fragment.
///
/// This macro takes XML, and produces an element fragment represented as
/// `Elements`. For example, the following APX snippet:
/// ```rust
/// apx!{
///     <my_component param1=123 param2=456/>
///	}
/// ```
/// Is syntactical sugar for the following:
/// ```rust
/// vec![
///     my_component().param1(123).param3(456)
/// ]
/// ```
/// And it would be used together with a component defined like this:
/// ```rust
/// #[derive_component(ComponentBuilder,Default,SnakeFactory)]
/// pub struct MyComponent {
///     param1: i32,
///     param2: i32
/// }
///
/// impl Element for MyComponent {
///     fn render(self:ElementWrap<Self>)->Elements {
///         // ...
///     }
/// }
#[proc_macro]
pub fn apx(input: TokenStream) -> TokenStream {
	apx::apx(input)
}
