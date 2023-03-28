use proc_macro2;
use proc_macro::{*};
use quote::quote;
use syn_rsx::{parse2, Node};
use syn::{Ident};

fn process_rsx_node(node: &Node)->proc_macro2::TokenStream {
	if let Node::Element(element)=&node {
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

		quote!(vec![Element::create(#name,#props{#attrs ..Default::default()},#children)])
	}

	else if let Node::Block(block)=&node {
		let value=block.value.as_ref();
		quote!(#value)
	}

	else {
		panic!("parse error");
	}
}

fn process_rsx_fragment(nodes: &Vec<Node>)->proc_macro2::TokenStream {
	let mut elements=quote!();
	for node in nodes {
		elements.extend(process_rsx_node(node));
		elements.extend(quote!(,));
	}

	quote!(flatten_elements(&mut vec![#elements]))
}

pub fn apx(input: TokenStream) -> TokenStream {
	let nodes=parse2(input.into()).unwrap();
	let tokens=process_rsx_fragment(&nodes);
	//println!("{}",tokens.to_string());
	tokens.into()
}
