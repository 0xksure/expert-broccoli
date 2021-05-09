extern crate proc_macro;
use proc_macro::*;
use quote::quote;
use std::collections::HashSet as Set;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn get_(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
