use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(ProtoWrapper)]
pub fn proto_wrapper_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_proto_wrapper_macro(&ast)
}

fn impl_proto_wrapper_macro(ast: &syn::DeriveInput) -> TokenStream {
    // println!("{:?}", ast);
    let name = &ast.ident;
    let proto_type = match &ast.data {
        syn::Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Unnamed(unnamed_field) => match &unnamed_field.unnamed.first().unwrap().ty {
                syn::Type::Path(p) => p,
                //  match p.path.get_ident() {
                //     Some(ident) => ident,
                //   None => panic!("Not implemented"),
                //  },
                _ => panic!("Expects Type::Path"),
            },
            _ => panic!("Expecting just one Field::Unnamed"),
        },
        _ => panic!("Expecting just one Data::Struct"),
    };

    let gen = quote! {
        impl ProtoWrapper<#proto_type> for #name {
            fn new_ref(proto: &#proto_type) -> &Self {
                (unsafe { &*(proto as *const #proto_type as *const Self) }) as _
            }

            fn inner_proto(&self) -> &#proto_type {
                &self.0
            }

            fn from_proto(proto: #proto_type) -> Self {
                Self(proto)
            }
        }

        impl From<#name> for #proto_type {
            fn from(val: #name) -> Self {
                val.0
            }
        }

        impl From<&#name> for #proto_type {
            fn from(val: &#name) -> Self {
                val.0.clone()
            }
        }
    };
    gen.into()
}
