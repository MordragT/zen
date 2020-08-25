extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(FromReader)]
pub fn from_reader_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_from_reader(&ast)
}

fn impl_from_reader(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl FromReader for #name {
            fn from_reader<R: Read>(mut reader: R) -> Self {
                let mut buf = [0_u8; std::mem::size_of::<Self>()];
                reader.read_exact(&mut buf).unwrap();
                bincode::deserialize(&buf).unwrap()
            }
        }
    };
    gen.into()
}
