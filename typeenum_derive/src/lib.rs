use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

fn single<T>(mut i: impl Iterator<Item = T>) -> Option<T> {
    let result = i.next()?;
    if i.next().is_some() {
        None
    } else {
        Some(result)
    }
}

/// Dervies `HasVariant` for an enum. **Note: it ignores variants
/// with multiple or no contents**. Currently does not support
/// generics nor lifetimes.
///
/// For example:
///
/// ```
/// #[derive(HasVariant)]
/// enum MyEnum {
///     Integer(i32),
///     String(String),
/// }
/// ```
///
/// `MyEnum` now implements `HasVariant<i32>` and `HasVariant<String>`.
#[proc_macro_derive(HasVariant)]
pub fn has_variant_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let data = match input.data {
        Data::Enum(data) => data,
        _ => {
            return "compile_error!(\"`TypeStruct` only accepts enums.\");"
                .parse()
                .unwrap();
        }
    };

    let name = input.ident;
    data.variants
        .into_iter()
        .filter_map(|variant| {
            let variant_name = variant.ident;
            let tokens = match variant.fields {
                Fields::Named(named) => {
                    let field = single(named.named.into_iter())?;
                    let ident = field.ident.unwrap();
                    let ty = field.ty;
                    quote! {
                        impl ::std::convert::From<#ty> for #name {
                            fn from(x: #ty) -> #name {
                                #name::#variant_name { #ident: x }
                            }
                        }

                        impl ::typeenum::HasVariant<#ty> for #name {
                            fn get(&self) -> Option<&#ty> {
                                match self {
                                    #name::#variant_name { #ident: x } => Some(x),
                                    _ => None,
                                }
                            }

                            fn get_mut(&mut self) -> Option<&mut #ty> {
                                match self {
                                    #name::#variant_name { #ident: x } => Some(x),
                                    _ => None,
                                }
                            }
                        }
                    }
                }
                Fields::Unnamed(unnamed) => {
                    let field = single(unnamed.unnamed.into_iter())?;
                    let ty = field.ty;
                    quote! {
                        impl ::std::convert::From<#ty> for #name {
                            fn from(x: #ty) -> #name {
                                #name::#variant_name(x)
                            }
                        }

                        impl ::typeenum::HasVariant<#ty> for #name {
                            fn get(&self) -> Option<&#ty> {
                                match self {
                                    #name::#variant_name(x) => Some(x),
                                    _ => None,
                                }
                            }

                            fn get_mut(&mut self) -> Option<&mut #ty> {
                                match self {
                                    #name::#variant_name(x) => Some(x),
                                    _ => None,
                                }
                            }
                        }
                    }
                }
                Fields::Unit => return None,
            };
            Some(TokenStream::from(tokens))
        })
        .collect()
}
