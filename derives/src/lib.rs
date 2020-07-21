extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::quote;
use std::any::Any;
use syn;
use syn::spanned::Spanned;

#[proc_macro_derive(ToParams)]
pub fn to_params_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_to_params(&ast)
}

fn type_to_string(ty: syn::Type) -> String {
    match ty {
        syn::Type::Path(_) => "Path",
        syn::Type::Array(_) => "Array",
        syn::Type::Verbatim(_) => "Verbatim",
        syn::Type::Tuple(_) => "Tuple",
        syn::Type::Reference(_) => "Reference",
        syn::Type::Group(_) => "Group",
        _ => "Other",
    }
    .to_string()
}

fn impl_to_params(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = match &ast.data {
        syn::Data::Struct(data) => data.fields.iter(),
        _ => panic!("only struct"),
    };

    let field_ident = fields.map(|f| {
        let ident = f.ident.as_ref().unwrap();

        let x = match f.ty.clone() {
            syn::Type::Path(t) => match t.path.segments.first().unwrap().ident.to_string().as_str()
            {
                "f32" => quote! { &self.#ident.to_le_bytes() },
                "u8" => quote! { &[self.#ident] },
                "bool" => quote! { &[(if self.#ident {1u8} else {0u8})] },
                _ => panic!("f32, u8, bool"),
            },
            _ => panic!("only single type"),
        };

        quote! {
            size += buf.write(#x).unwrap();
        }
    });

    let gen = quote! {
        impl ToParams for #name {
            fn to_params(&self) -> (usize, [u8; PARAMS_SIZE]) {
                let mut b = [0u8; PARAMS_SIZE];
                let mut size = 0;
                {
                    let mut buf = &mut b[0..PARAMS_SIZE];
                    #(#field_ident)*
                    buf.flush().unwrap()
                }
                (size, b)
            }
        }
    };
    gen.into()
}
