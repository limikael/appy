use proc_macro::{*};
use quote::quote;
use syn::{
	parse_macro_input, ItemFn, Ident, FnArg, TypePath,
	PatType, Type, Path
};

pub fn function_component(_attr: TokenStream, input: TokenStream) -> TokenStream {
	let ast=parse_macro_input!(input as ItemFn);
	let name=ast.sig.ident.clone();
	let alias_name=format!("Props_{}",name.to_string());
	let alias_ident=Ident::new(&alias_name,Span::call_site().into());

	let arg_type=match ast.sig.inputs.first().unwrap() {
		FnArg::Typed(PatType{ty, ..})=>{
			match &**ty {
				Type::Path(TypePath{path: Path{segments, ..}, ..})=>{
					segments.first().unwrap().ident.clone()
				},
				_=>{panic!("unexpected")}
			}
		}
		_=>{panic!("uexpected")}
	};

	TokenStream::from(quote!{
		pub type #alias_ident=#arg_type;
		#ast
	})
}
