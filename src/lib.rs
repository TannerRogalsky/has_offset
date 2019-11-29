extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{self, parse_macro_input, Data, DeriveInput, Fields};
use quote::{quote, format_ident};

#[proc_macro_derive(Offsets)]
pub fn derive_offsets(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    match input.data {
        Data::Struct(s) => {
            match s.fields {
                Fields::Named(fields) => {
                    let field_types = fields.named.iter().scan(vec![], |state, field| {
                        state.push(&field.ty);
                        Some(state.clone())
                    });
                    let field_names = fields.named.iter().map(|field| {
                        format_ident!("get_{}_offset", field.ident.as_ref().unwrap())
                    });
                    let getters = field_types.into_iter().zip(field_names).map(|(mut types, ident)| {
                        types.pop();
                        if types.is_empty() {
                            quote! {
                                const fn #ident() -> usize {
                                    0usize
                                }
                            }
                        } else {
                            quote! {
                                const fn #ident() -> usize {
                                    #(::std::mem::size_of::<#types>())+*
                                }
                            }
                        }
                    });
                    let ident = format_ident!("{}", input.ident);
                    TokenStream::from(quote! {
                        impl #ident {
                            #(#getters)*
                        }
                    })
                },
                Fields::Unnamed(_) | Fields::Unit => panic!("only named fields are supported"),
            }
        }
        Data::Enum(_) | Data::Union(_) => panic!("only structs are supported"),
    }
}
