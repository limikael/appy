use proc_macro::{*};
use quote::{quote,format_ident};
use syn::{parse_macro_input, ItemFn, FnArg, PatType};

pub fn function_component(_attr: TokenStream, input: TokenStream) -> TokenStream {
	let mut ast=parse_macro_input!(input as ItemFn);

	let impl_generics=ast.sig.generics.clone();
	//let where_clause=ast.sig.where_clause.clone(); // what to do with the where clause?

	ast.sig.ident=format_ident!("_{}",ast.sig.ident.clone().to_string());
	let name=ast.sig.ident.clone();

	let arg_type=match ast.sig.inputs.first().unwrap() {
		FnArg::Typed(PatType{ty, ..})=>{
			ty
		}
		_=>{panic!("uexpected")}
	};

	let out=quote!{
		impl #impl_generics appy::types::RenderElement for #arg_type {
			fn render(self:appy::types::ElementWrap<Self>)->appy::types::Elements {
				#name(*self)
			}
		}
		#ast
	};

//	println!("{:?}",out.to_string());

	TokenStream::from(out)
}