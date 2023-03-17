use proc_macro::TokenStream;
use std::str::FromStr;
use minidom::{Element, Children};
use syn::{parse_macro_input, DeriveInput, parse::Parser};
use quote::quote;

#[proc_macro_attribute]
pub fn component(_attr: TokenStream, input: TokenStream) -> TokenStream {
	let mut ast = parse_macro_input!(input as DeriveInput);
	match &mut ast.data {
		syn::Data::Struct(ref mut struct_data) => {           
			match &mut struct_data.fields {
				syn::Fields::Named(fields) => {
					fields
						.named
						.push(syn::Field::parse_named.parse2(quote! { pub children: ComponentFragment }).unwrap());
				}   
				_ => {
					()
				}
			}              

			return quote! {
				#[derive(Clone)]
				#ast
			}.into();
		}
		_ => panic!("`component` has to be used with structs "),
	}
}

fn parse_xml_token_stream(input: TokenStream)->Element {
	let mut s="".to_owned();
	for i in input {
		let tok=&i.to_string();
		s+=&tok;

		if tok!="<" && tok!="/" {
			s+=&" ";
		}
	}

	let mut xml_source="<fragment xmlns=\"apx\">".to_owned();
	xml_source+=&s;
	xml_source+=&"</fragment>";

	xml_source.parse().unwrap()
}

fn process_fragment_to_vec(fragment_els: Children)->String {
	let mut res="vec![".to_owned();

	let mut fragment_parts:Vec<String>=vec![];
	for el in fragment_els {
		let mut s="".to_owned();
		s+=&format!("Rc::new({}{{",el.name());

		let mut have_children:bool=false;
		let mut attr_parts:Vec<String>=vec![];
		for (key, val) in el.attrs() {
			attr_parts.push(format!("{}: {}",key,val));
			if key=="children" {
				have_children=true;
			}
		}

		if !have_children {
			attr_parts.push(format!("children: {}",process_fragment_to_vec(el.children())));
		}

		s+=&attr_parts.join(",");
		s+=&format!("}})");

		fragment_parts.push(s);
	}

	res+=&fragment_parts.join(",");

	res+="]";
	res
}

#[proc_macro]
pub fn apx(input: TokenStream) -> TokenStream {
	let root=parse_xml_token_stream(input);

	//println!("{:?}", root);
	let s=process_fragment_to_vec(root.children());
	//println!("{:?}", s);

	TokenStream::from_str(&s).unwrap()
}
