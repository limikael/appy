use proc_macro::{*};
use quote::quote;
use syn::{parse_macro_input, ItemFn};

pub fn appy_main(_attr: TokenStream, input: TokenStream) -> TokenStream {
	let ast=parse_macro_input!(input as ItemFn);
	let name=ast.sig.ident.clone();

	TokenStream::from(quote!{
		#ast
		pub fn main() {
			Appy::run(#name);
		}

		#[no_mangle]
		#[allow(non_snake_case)]
		pub fn SDL_main() {
			Appy::run(#name);
		}
	})
}
