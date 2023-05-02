use proc_macro::{*};
use quote::quote;
use syn::{
	parse_macro_input, Ident, ItemStruct, 
};
use convert_case::{Case, Casing};

pub fn snake_factory(input: TokenStream) -> TokenStream {
	let ast:ItemStruct=parse_macro_input!(input as ItemStruct);

	let struct_ident=ast.ident.clone();
	let generics = &ast.generics;
	let (_impl_generics, ty_generics, _where_clause) = generics.split_for_impl();
	let func_ident=Ident::new(&struct_ident.to_string().to_case(Case::Snake),struct_ident.span());

	let out=quote!{
    	/// Alias for ::new() on the corresponding struct.
        pub fn #func_ident #ty_generics()->appy::types::ElementWrap<#struct_ident #ty_generics> {
			#struct_ident::new()
        }
	};

	//println!("*********** macro out: {:?}",out.to_string());

	TokenStream::from(out)
}
