extern crate rest_client_codegen;
pub use rest_client_codegen::rest;
use std::*;

pub trait ClientMethods {
    fn get(parameters: Vec<impl fmt::Display>) -> Result<Box<Self>, Box<error::Error>>;
}

pub trait ClientVecMethods {
    fn gets(parameters: Vec<impl fmt::Display>) -> Result<Vec<Box<Self>>, Box<error::Error>>;
}
