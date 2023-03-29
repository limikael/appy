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
pub fn main_window(attr: TokenStream, input: TokenStream) -> TokenStream {
	main_window::main_window(attr,input)
}

mod function_component;
#[proc_macro_attribute]
pub fn function_component(attr: TokenStream, input: TokenStream) -> TokenStream {
	function_component::function_component(attr,input)
}

mod apx;
#[proc_macro]
pub fn apx(input: TokenStream) -> TokenStream {
	apx::apx(input)
}
