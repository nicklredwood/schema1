use proc_macro::TokenStream;

// use proc_macro2::{
// 	// Span as Span2,
// 	// Ident as Ident2,
// 	TokenStream as TokenStream2,
// };
use quote::quote;
use syn::{
	parse_macro_input,
	DeriveInput,
	// Item,
	// Ident,
};

use uuid::Uuid;

// ===

/// The UUID namespace derived from the implemented type
pub(crate) trait Namespace {
	const NS: Uuid;
}

/// > [!caution]
/// > Not to be confused with `uuid::uuid!()`
/// A "root" namespace is created if no "in $ns" is provided.
macro_rules! __uuid (
	($data:ident in $ns:ident) => {
		Uuid::new_v5(&uuid::uuid!($ns), $data.as_bytes())
	};
	($data:ident) => {
		Uuid::new_v5(&Uuid::max(), $data.as_bytes())
	};
);

/// Defines the namespace this type belongs to
#[proc_macro_attribute]
pub fn namespace(attr: TokenStream, item: TokenStream) -> TokenStream {
	let attr_ast = parse_macro_input!(attr as DeriveInput);
	// let attr = TokenStream2::from(attr);
	let item_ast = parse_macro_input!(item as DeriveInput);
	// let item = TokenStream2::from(item);

	let data = format!("{:?}", item.clone()); // This needs to be more clever
	let name = &item_ast.ident;
	// if let ns =
	let expr: Uuid = if attr_ast.attrs.is_empty() {
	    __uuid!(data)
	} else {
	    let ns = &attr_ast.ident;
	    __uuid!(data in ns)
	};
	quote! {
		impl Namespace for #name {
			const NS: uuid::Uuid = uuid::uuid!(#expr);
		}
	}.into()
}

// /// Derives a UUID namespace from this type, which other types or data may belong to
// /// Not necessary what with the #[namespace] attribute, below
// #[proc_macro_derive(Namespace)]
// pub fn derive_uuid(input: TokenStream) -> TokenStream {
//	 // let input = TokenStream2::from(input);
// 	// Convert input to string
// 	// let s = input.to_string();
// 	// Parse the string representation
// 	// let ast = syn::parse_derive_input(&s).unwrap();
// 	let ast = parse_macro_input!(input as DeriveInput);
// 	let name = &ast.ident;
// 	let namespace = ns!(name);
// 	let data;
// 	static expr: uuid::Uuid = uuid!(data in namespace);
// 	// Generate the implementation
// 	quote! {
// 		impl Namespace for #name {
// 			const NS: uuid::Uuid = #expr;
// 		}
// 	}.into()
// }
