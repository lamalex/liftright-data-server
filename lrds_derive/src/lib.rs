extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(ExtractUser)]
pub fn extract_user_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_extract_user(&ast)
}

fn impl_extract_user(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        impl ExtractUser for #name {
            fn extract_user(&self) -> User {
                User::new(self.device_id)
            }
        }
    };

    gen.into()
}
