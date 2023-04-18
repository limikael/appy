use proc_macro::{*};
use quote::quote;
use syn::{
	parse_macro_input, Ident, ItemStruct, 
};
use convert_case::{Case, Casing};

pub fn snake_factory(input: TokenStream) -> TokenStream {
	let ast:ItemStruct=parse_macro_input!(input as ItemStruct);

	let struct_ident=ast.ident.clone();
	let func_ident=Ident::new(&struct_ident.to_string().to_case(Case::Snake),struct_ident.span());

	let out=quote!{
        pub fn #func_ident()->ElementWrap<#struct_ident> {
			#struct_ident::new()
        }
	};

	//println!("*********** macro out: {:?}",out.to_string());

	TokenStream::from(out)
}
