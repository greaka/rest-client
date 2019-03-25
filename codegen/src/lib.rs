extern crate proc_macro;
extern crate proc_macro2;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::{
    parse::{Parse, ParseStream},
    Attribute, Ident, LitStr, Token,
};

struct Attr {
    uri: Vec<Attribute>,
    vec: bool,
}

impl Parse for Attr {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        let uri = input.call(Attribute::parse_inner)?;
        //let uri: LitStr = input.parse()?;
        let mut is_vec = false;
        if let Ok(_) = input.parse::<Token![,]>() {
            if let Ok(vec) = input.parse::<Ident>() {
                match format!("{}", vec).as_ref() {
                    "vec" => is_vec = true,
                    _ => {}
                }
            }
        }
        Ok(Attr { uri, vec: is_vec })
    }
}

#[proc_macro_attribute]
pub fn rest(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_copy = item.clone();
    let input = syn::parse_macro_input!(item_copy as syn::ItemStruct);
    let item: proc_macro2::TokenStream = item.into();
    let ident = &input.ident;
    let input = syn::parse_macro_input!(attr as Attr);
    let result = quote! {
        #[derive(Serialize, Deserialize)]
        #item

        impl ClientMethods for #ident {
        }
    };
    result.into()
}

trait ClientMethods {
    type Future: hyper::rt::Future;
    fn get<Future>(parameters: Vec<&str>) -> Future;
}
