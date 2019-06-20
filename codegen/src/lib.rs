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

    let mut is_vec = false;
    let mut wrapper: Option<syn::Ident> = None;
    for arg in args[1..].iter() {
        match arg {
            syn::NestedMeta::Meta(syn::Meta::Word(ref fname))
                if fname == &syn::Ident::new("vec", proc_macro2::Span::call_site()) =>
            {
                is_vec = true;
            }
            syn::NestedMeta::Meta(syn::Meta::NameValue(ref value))
                if value.ident == syn::Ident::new("wrapper", proc_macro2::Span::call_site()) =>
            {
                let lit = if let syn::Lit::Str(name) = &value.lit {
                    name
                } else {
                    panic!("wrapper has not the format wrapper = \"Type\"");
                };
                wrapper = Some(syn::Ident::new(&lit.value(), lit.span()));
            }
            _ => {}
        }
    }

    let item_copy = item.clone();
    let input = syn::parse_macro_input!(item_copy as syn::ItemStruct);
    let item: proc_macro2::TokenStream = item.into();
    let ident = &input.ident;

    let mut result_type = if is_vec {
        quote! { Vec<Box<Self>> }
    } else {
        quote! { Box<Self> }
    };

    if let Some(wrapper) = wrapper {
        result_type = quote! { #wrapper<#result_type> };
    };

    let final_trait = quote! { ClientMethods<#result_type> };

    let count = path.matches("{}").count();
    let mut counter_vec = Vec::new();
    for _i in 0..count {
        counter_vec.push(quote! { iter.next().unwrap() });
    }

    let result = quote! {
        impl #final_trait for #ident {
            fn get(parameters: impl IntoIterator<Item = impl std::fmt::Display>) -> Result<#result_type, Box<std::error::Error>> {
                let mut iter = parameters.into_iter();
                let request_path = format!(#path, #(#counter_vec),*);
                let new_self: #result_type = reqwest::get(&request_path)?
                    .json()?;
                Ok(new_self)
            }
        }
    };

    (quote! {
        #item
        #result
    })
    .into()
}
