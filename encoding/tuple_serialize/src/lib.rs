// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, Ident};

#[proc_macro_derive(TupleSerialize, attributes(tuple_order))]
pub fn tuple_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let fields: &Punctuated<syn::Field, syn::token::Comma> = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    let _ordered: Vec<Ident> = vec![];
    let _fields_vec: Vec<Ident> = fields
        .iter()
        // This could probably be cleaner
        .filter(|f| f.ident.is_some())
        .map(|f| f.ident.clone().unwrap())
        .collect();

    // println!("{:#?}", fields_vec);

    // TODO just serialize/deserialize fields based on parsed
    let expanded = quote! {
        impl serde::ser::Serialize for #struct_name {
            fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
            where
                S: serde::ser::Serializer,
            {
                (&self.field1, &self.field2, &self.field3).serialize(s)
            }
        }
        impl<'de> serde::de::Deserialize<'de> for #struct_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                let (field1, field2, field3) = serde::Deserialize::deserialize(deserializer)?;
                Ok(Self {
                    field1,
                    field2,
                    field3,
                })
            }
        }
    };

    expanded.into()
}
