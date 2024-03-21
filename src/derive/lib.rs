// Copyright (c) Tribufu. All Rights Reserved.
// SPDX-License-Identifier: MIT

mod config_trait;

use proc_macro::TokenStream;

#[proc_macro_derive(Config)]
pub fn derive_config(input: TokenStream) -> TokenStream {
    config_trait::derive(input)
}
