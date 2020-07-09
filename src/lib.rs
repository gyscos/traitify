extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemEnum};

#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    item
}

#[proc_macro_attribute]
pub fn traitify(args: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemEnum);

    let enum_name = &input.ident;
    let mut trait_name = syn::Ident::new(
        &format!("{}Trait", enum_name),
        syn::export::Span::call_site(),
    );

    let args = parse_macro_input!(args as AttributeArgs);

    for arg in args {
        if let syn::NestedMeta::Meta(syn::Meta::NameValue(syn::MetaNameValue {
            path, lit, ..
        })) = arg
        {
            if path.is_ident("name") {
                if let syn::Lit::Str(name) = lit {
                    trait_name = syn::Ident::new(&name.value(), syn::export::Span::call_site());
                }
            }
        }
    }

    let gen = &input.generics;

    let items: proc_macro2::TokenStream = input
        .variants
        .iter()
        .map(|v| {
            let name = &v.ident;
            let fields = &v.fields;
            quote!(
                struct #name #gen #fields;

                impl #gen #trait_name for #name #gen {
                    fn with<F, R>(&self, f: F) -> R where F: FnOnce(&#enum_name #gen) -> R {
                        f(&#enum_name::#name)
                    }
                }
            )
        })
        .collect();

    TokenStream::from(quote!(
        #input

        trait #trait_name {
            fn with<F, R>(&self, f: F) -> R where F: FnOnce(&#enum_name) -> R;
        }

        impl #trait_name for #enum_name {
            fn with<F, R>(&self, f: F) -> R where F: FnOnce(&#enum_name) -> R {
                f(self)
            }
        }

        #items
    ))
}
