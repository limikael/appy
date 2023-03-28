use proc_macro::{*};

/*#[proc_macro_derive(Props)]
pub fn derive_props(input: TokenStream) -> TokenStream {
	let ast=parse_macro_input!(input as DeriveInput);
	let name=ast.ident;

	TokenStream::from(quote!{
		impl Props for #name {}
	})
}*/

mod appy_main;
#[proc_macro_attribute]
pub fn appy_main(attr: TokenStream, input: TokenStream) -> TokenStream {
	appy_main::appy_main(attr,input)
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
