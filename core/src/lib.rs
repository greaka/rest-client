#![feature(trace_macros)]
trace_macros!(true);
#[cfg(test)]
mod tests {
    extern crate rest_client_codegen;
    use rest_client_codegen::*;
    #[rest("foo", vec)]
    struct A {}
}
