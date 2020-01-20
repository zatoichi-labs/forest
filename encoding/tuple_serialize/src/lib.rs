extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(TupleSerialize, attributes(tuple_serialize))]
pub fn tuple_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let b_name = format!("Tuple{}", struct_name);
    let _b_ident = syn::Ident::new(&b_name, struct_name.span());

    let expanded = quote! {};

    expanded.into()
}
