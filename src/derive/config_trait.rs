// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: MIT

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_ident = &input.ident;

    let expanded = quote! {
        impl Config for #struct_ident {}
    };

    expanded.into()
}
