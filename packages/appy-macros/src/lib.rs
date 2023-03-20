use proc_macro2;
use proc_macro::{*};
use quote::quote;
use syn_rsx::{parse2, Node};
use syn::{
	parse_macro_input, DeriveInput, ItemFn, Ident, FnArg, TypePath,
	PatType, Type, Path
};

#[proc_macro_attribute]
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

#[proc_macro_derive(Props)]
pub fn derive_props(input: TokenStream) -> TokenStream {
	let ast=parse_macro_input!(input as DeriveInput);
	let name=ast.ident;

	TokenStream::from(quote!{
		impl Props for #name {}
	})
}

fn process_rsx_node(node: &Node)->proc_macro2::TokenStream {
	let Node::Element(element) = &node else { panic!("parse error") };

	let mut attrs=quote!();
	for attr_element in &element.attributes {
		let Node::Attribute(attr)=attr_element else { panic!("parse error") };
		let key=&attr.key;
		let value=attr.value.as_ref().unwrap().as_ref();
		attrs.extend(quote!(#key: #value,));
	}

	let name=&element.name;
	let props=Ident::new(&format!("Props_{}",name.to_string()),Span::call_site().into());
	let children=process_rsx_fragment(&element.children);

	quote!(Element::create(#name,#props{#attrs ..Default::default()},#children))
}

fn process_rsx_fragment(nodes: &Vec<Node>)->proc_macro2::TokenStream {
	let mut elements=quote!();
	for node in nodes {
		elements.extend(process_rsx_node(node));
		elements.extend(quote!(,));
	}

	quote!(vec![#elements])
}

#[proc_macro]
pub fn apx(input: TokenStream) -> TokenStream {
	let nodes=parse2(input.into()).unwrap();
	process_rsx_fragment(&nodes).into()
}

