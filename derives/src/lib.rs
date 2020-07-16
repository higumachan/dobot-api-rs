extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;


#[proc_macro_derive(ToParams)]
pub fn to_params_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_to_params(&ast)
}


fn impl_to_params(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = match &ast.data {
        syn::Data::Struct(data) => {
            data.fields.iter()
        }
        _ => panic!("only struct")
    };

    let field_ident = fields.map(|f| {
        let ident = f.ident.as_ref().unwrap();


        quote! {
            size += buf.write(&self.#ident.to_le_bytes()).unwrap();
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
