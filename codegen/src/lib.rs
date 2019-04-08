extern crate proc_macro;
extern crate proc_macro2;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_attribute]
pub fn rest(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(attr as syn::AttributeArgs);
    if args.is_empty() {
        panic!("invalid number of arguments");
    }

    // https://github.com/actix/actix-web/blob/1970c99522ef37d4a5fbed404b9b100912fad69a/actix-web-codegen/src/lib.rs#L18
    let path = match args[0] {
        syn::NestedMeta::Literal(syn::Lit::Str(ref fname)) => {
            let fname = quote!(#fname).to_string();
            fname.as_str()[1..fname.len() - 1].to_owned()
        }
        _ => panic!("resource path"),
    };

    let count = path.matches("{}").count();
    let count_iter = 0..count;

    let mut is_vec = false;
    for arg in args[1..].iter() {
        match arg {
            syn::NestedMeta::Meta(syn::Meta::Word(ref fname))
                if fname == &syn::Ident::new("vec", proc_macro2::Span::call_site()) =>
            {
                is_vec = true;
            }
            _ => {}
        }
    }

    let item_copy = item.clone();
    let input = syn::parse_macro_input!(item_copy as syn::ItemStruct);
    let item: proc_macro2::TokenStream = item.into();
    let ident = &input.ident;

    let result = if !is_vec {
        quote! {
            impl ClientMethods for #ident {
                fn get(parameters: Args) -> std::future::Future<Output = Self> {
                    let request_path = format!(#path, #(parameters.#count_iter),*);
                }
            }
        }
    } else {
        quote! {
            impl ClientVecMethods for #ident {
                fn get(parameters: Args) -> std::future::Future<Output = Vec<Box<Self>>> {
                    let request_path = format!(#path, #(parameters.#count_iter),*);
                }
            }
        }
    };

    (quote! {
        #item
        #result
    })
    .into()
}
