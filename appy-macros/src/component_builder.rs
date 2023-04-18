use proc_macro::*;
use quote::quote;
use syn::{
	parse_macro_input, ItemStruct, Fields::Named
};
//use convert_case::{Case, Casing};

pub fn component_builder(input: TokenStream) -> TokenStream {
	let mut ast:ItemStruct=parse_macro_input!(input as ItemStruct);
	let mut builder_body=quote!{};

	ast.fields=Named(if let Named(fields)=ast.fields {
		for field in &fields.named {
			let ident=field.ident.as_ref().unwrap();
			let ty=&field.ty;

			if let syn::Type::Path(type_path)=&field.ty {
				if type_path.path.segments.len()!=1 {
					panic!("Expected type path of length 1");
				}
				let i=type_path.path.segments[0].ident.clone();

				if i.to_string()=="Option" {
					if let syn::PathArguments::AngleBracketed(a)=&type_path.path.segments[0].arguments {
						//println!("{:?}",a.args[0]);
						let ty=&a.args[0];
						builder_body.extend(quote!{
							pub fn #ident(mut self:ElementWrap<Self>, val: #ty)->ElementWrap<Self> {
								self.#ident=Some(val);
								self
							}
						})
					}

					else {panic!("expected generic argiments for option")}
				} else {
					builder_body.extend(quote!{
						pub fn #ident(mut self:ElementWrap<Self>, val: #ty)->ElementWrap<Self> {
							self.#ident=val;
							self
						}
					})
				}
			} else {panic!("expected type path")};
		}

		fields
	} else {panic!("parse error")});

	let struct_ident=ast.ident.clone();
	builder_body=quote!{
		impl #struct_ident {
			pub fn new()->ElementWrap<Self> {
				ElementWrap::new(Self::default())
			}

            #builder_body
        }
	};

	TokenStream::from(builder_body)
}