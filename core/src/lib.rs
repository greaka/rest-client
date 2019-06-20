extern crate rest_client_codegen;
pub use rest_client_codegen::rest;
use std::*;

pub trait ClientMethods<T> {
        fn get(
                parameters: impl IntoIterator<Item = impl fmt::Display>,
        ) -> Result<T, Box<error::Error>>;
}
