use proc_macro::{*};
use quote::quote;
use syn::{
	parse_macro_input, ItemStruct, parse::Parser, Fields::Named
};
//use convert_case::{Case, Casing};

pub fn derive_component(attr_stream: TokenStream, input: TokenStream) -> TokenStream {
	let mut ast:ItemStruct=parse_macro_input!(input as ItemStruct);

	let mut attr_out=quote!();
	for a in attr_stream {
		match a {
			TokenTree::Ident(i)=>{
				let out_i=proc_macro2::Ident::new(&*i.to_string(),ast.ident.span());
				attr_out.extend(quote!(#out_i));
			},
			TokenTree::Punct(ch)=>{
				let out_p=proc_macro2::Punct::new(ch.as_char(),proc_macro2::Spacing::Alone);
				attr_out.extend(quote!(#out_p));
			}
			_=>{}
		}
	}

	ast.fields=Named(if let Named(mut fields)=ast.fields {
		let p=syn::Field::parse_named.parse2(quote!{
			children: Elements
		});

		fields.named.push(p.unwrap());

		let p=syn::Field::parse_named.parse2(quote!{
			key: Option<String>
		});

		fields.named.push(p.unwrap());

		fields
	} else {panic!("parse error")});

	let struct_ident=&ast.ident;
	let generics=&ast.generics;
	let (impl_generics, ty_generics, where_clause)=generics.split_for_impl();

	let out=quote!{
		#[derive(#attr_out)]
		#ast

		impl #impl_generics appy::types::Element for #struct_ident #ty_generics #where_clause {
		    fn get_key(&self)->Option<String> {
		    	self.key.clone()
		    }
		}
	};

	//println!("*********** macro out: {:?}",out.to_string());

	TokenStream::from(out)
}
