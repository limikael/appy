use proc_macro::*;
use quote::{quote,format_ident};
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
							pub fn #ident(
									mut self:appy::types::ElementWrap<Self>,
									val: #ty)
										->appy::types::ElementWrap<Self> {
								self.#ident=Some(val);
								self
							}
						});

						let ident_option=format_ident!("{}{}",ident,"_option");
						builder_body.extend(quote!{
							pub fn #ident_option(
									mut self:appy::types::ElementWrap<Self>, 
									val: Option<#ty>)
										->appy::types::ElementWrap<Self> {
								self.#ident=val;
								self
							}
						});
					}

					else {panic!("expected generic arguments for option")}
				} else if i.to_string()=="String" {
					builder_body.extend(quote!{
						pub fn #ident(mut self:appy::types::ElementWrap<Self>, val: &str)->appy::types::ElementWrap<Self> {
							self.#ident=val.to_string();
							self
						}
					});
				} else if i.to_string()=="Dim" {
					builder_body.extend(quote!{
						pub fn #ident<T>(mut self:appy::types::ElementWrap<Self>, val: T)->appy::types::ElementWrap<Self>
								where appy::types::Dim:From<T> {
							self.#ident=appy::types::Dim::from(val);
							self
						}
					});
				} else {
					builder_body.extend(quote!{
						pub fn #ident(mut self:appy::types::ElementWrap<Self>, val: #ty)->appy::types::ElementWrap<Self> {
							self.#ident=val;
							self
						}
					});
				}
			} else {panic!("expected type path")};
		}

		fields
	} else {panic!("parse error")});

	let struct_ident=&ast.ident;
	let generics = &ast.generics;
	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

	builder_body=quote!{
		impl #impl_generics #struct_ident #ty_generics #where_clause {
			pub fn new()->appy::types::ElementWrap<Self> {
				appy::types::ElementWrap::new(Self::default())
			}

            #builder_body
        }
	};

	//println!("{:?}",builder_body.to_string());

	TokenStream::from(builder_body)
}