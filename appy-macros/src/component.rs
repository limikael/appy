use proc_macro::{*};
use quote::quote;
use syn::{
	parse_macro_input, Ident, ItemStruct, parse::Parser, Fields::Named
};
use convert_case::{Case, Casing};

pub fn component(_attr: TokenStream, input: TokenStream) -> TokenStream {
	let mut ast:ItemStruct=parse_macro_input!(input as ItemStruct);
	let mut builder_body=quote!{};

	ast.fields=Named(if let Named(mut fields)=ast.fields {
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

		let p=syn::Field::parse_named.parse2(quote!{
			children: Elements
		});

		fields.named.push(p.unwrap());
		fields
	} else {panic!("parse error")});

	builder_body.extend(quote!{
		pub fn new()->ElementWrap<Self> {
			ElementWrap::new(Self::default())
		}
		pub fn children(mut self:ElementWrap<Self>, val: Elements)->ElementWrap<Self> {
			self.children=val;
			self
		}
		pub fn child(mut self:ElementWrap<Self>, val: ElementWrap<dyn Element>)->ElementWrap<Self> {
			self.children.push(val);
			self
		}
	});

	let struct_ident=ast.ident.clone();
	let func_ident=Ident::new(&struct_ident.to_string().to_case(Case::Snake),struct_ident.span());

	builder_body=quote!{
		impl #struct_ident {
            #builder_body
        }

        pub fn #func_ident()->ElementWrap<#struct_ident> {
			#struct_ident::new()
        }
	};

	let out=quote!{
		#[derive(Default)]
		#ast
		#builder_body
	};

	//println!("*********** macro out: {:?}",out.to_string());

	TokenStream::from(out)
}
