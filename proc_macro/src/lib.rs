#![feature(proc_macro)]
#![recursion_limit="128"]
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;


#[proc_macro]
pub fn route_info(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let literal_str: syn::LitStr = syn::parse(input).expect("Syntax error, expected a string literal");

    // Build the impl
    let gen = impl_route_info(&literal_str);
    // Return the generated impl
    gen.into()
}

fn impl_route_info( route_literal: &syn::LitStr) -> quote::Tokens {
    let route: String = route_literal.value();

    // Perform some validation on the string before parsing it.
    if let Some(first_character) = route.chars().next() {
        if first_character != '/' {
            panic!("The provided route did not start with a slash.")
        }
    } else {
        panic!("The provided route was empty.")
    }

    // TODO Running `RouteInfo::parse()` here would be nice as well.

    quote! {
        RouteInfo::parse(#route_literal).expect("Could not parse route")
    }
}

