use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemFn};

#[proc_macro_derive(Component)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl Component for #ident {

        }
    };
    output.into()
}

#[proc_macro_attribute]
pub fn system(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(input as ItemFn);

    let ident = item.sig.ident;
    let output = quote! {
        fn #ident() -> i64 {
            return 42
        }
    };

    output.into()
}