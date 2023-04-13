use proc_macro::{*};

/*#[proc_macro_derive(Props)]
pub fn derive_props(input: TokenStream) -> TokenStream {
	let ast=parse_macro_input!(input as DeriveInput);
	let name=ast.ident;

	TokenStream::from(quote!{
		impl Props for #name {}
	})
}*/

mod main_window;
#[proc_macro_attribute]
/// Application entry point.
///
/// The main window macro defines the main application entry point.
/// It should be placed on a function that returns Elements. In this
/// sense, it is similar to a [macro@function_component], but it
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

mod function_component;
/// Create a function component.
///
/// A function component takes input in the form of props and children,
/// and return elements describing what should appear on the screen.
///
/// # Example
/// ```rust
/// pub struct FuncProps {
///   param1: i32,
///   param2: i32
/// }
///
/// #[function_component]
/// pub fn a_func(p:FuncProps, c:Elements)->Elements {
///   apx! {
///     <bg col=0x000000/>
///   }
/// }
/// ```
///
/// The function component declared above can be used together with the
/// apx macro:
/// ```rust
/// apx!{
///   <a_func param1=123 param2=456/>
///	}
/// ```
#[proc_macro_attribute]
pub fn function_component(attr: TokenStream, input: TokenStream) -> TokenStream {
	function_component::function_component(attr,input)
}

mod apx;
/// Process element fragment.
///
/// This macro takes XML, and produces an element fragment represented as
/// Elements. The node names in the XML refers to a [macro@function_component].
/// The attributes refers to the members to the function's props struct.
/// For example, the following APX snippet:
///
/// ```rust
/// apx!{
///   <a_func param1=123 param2=456/>
///	}
/// ```
///
/// Is intended to be used with a function component declared as:
///
/// ```rust
/// pub struct FuncProps {
///   param1: i32,
///   param2: i32
/// }
///
/// #[function_component]
/// pub fn a_func(p:FuncProps, c:Elements)->Elements {
///   // ...
///}
#[proc_macro]
pub fn apx(input: TokenStream) -> TokenStream {
	apx::apx(input)
}
