#![feature(futures_api)]
#![feature(associated_type_defaults)]

extern crate rest_client_codegen;
pub use rest_client_codegen::rest;

pub trait ClientMethods {
    fn get(parameters: Args) -> Result<Box<Self>, Box<std::error::Error>>;
}

pub trait ClientVecMethods {
    fn gets(parameters: Args) -> Result<Vec<Box<Self>>, Box<std::error::Error>>;
}

#[derive(Debug)]
pub struct ParameterError;
impl std::fmt::Display for ParameterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Wrong Parameter")
    }
}
impl std::error::Error for ParameterError {}

pub enum Args {
    OneArg((String,)),
    TwoArgs((String, String)),
    ThreeArgs((String, String, String)),
    FourArgs((String, String, String, String)),
    FiveArgs((String, String, String, String, String)),
    SixArgs((String, String, String, String, String, String)),
    SevenArgs((String, String, String, String, String, String, String)),
    EightArgs(
        (
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
        ),
    ),
    NineArgs(
        (
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
        ),
    ),
    TenArgs(
        (
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
        ),
    ),
}

impl Args {
    pub fn new<T>(args: T) -> Args
    where
        T: Into<Args>,
    {
        args.into()
    }
}

impl<T> From<(T,)> for Args
where
    T: std::fmt::Display + std::marker::Sized,
{
    fn from(arg: (T,)) -> Args {
        Args::OneArg((format!("{}", arg.0),))
    }
}

impl<T> From<(T, T)> for Args
where
    T: std::fmt::Display + std::marker::Sized,
{
    fn from(args: (T, T)) -> Args {
        Args::TwoArgs((format!("{}", args.0), format!("{}", args.1)))
    }
}

impl<T> From<(T, T, T)> for Args
where
    T: std::fmt::Display + std::marker::Sized,
{
    fn from(args: (T, T, T)) -> Args {
        Args::ThreeArgs((
            format!("{}", args.0),
            format!("{}", args.1),
            format!("{}", args.2),
        ))
    }
}

impl<T> From<(T, T, T, T)> for Args
where
    T: std::fmt::Display + std::marker::Sized,
{
    fn from(args: (T, T, T, T)) -> Args {
        Args::FourArgs((
            format!("{}", args.0),
            format!("{}", args.1),
            format!("{}", args.2),
            format!("{}", args.3),
        ))
    }
}

impl<T> From<(T, T, T, T, T)> for Args
where
    T: std::fmt::Display + std::marker::Sized,
{
    fn from(args: (T, T, T, T, T)) -> Args {
        Args::FiveArgs((
            format!("{}", args.0),
            format!("{}", args.1),
            format!("{}", args.2),
            format!("{}", args.3),
            format!("{}", args.4),
        ))
    }
}

impl<T> From<(T, T, T, T, T, T)> for Args
where
    T: std::fmt::Display + std::marker::Sized,
{
    fn from(args: (T, T, T, T, T, T)) -> Args {
        Args::SixArgs((
            format!("{}", args.0),
            format!("{}", args.1),
            format!("{}", args.2),
            format!("{}", args.3),
            format!("{}", args.4),
            format!("{}", args.5),
        ))
    }
}

impl<T> From<(T, T, T, T, T, T, T)> for Args
where
    T: std::fmt::Display + std::marker::Sized,
{
    fn from(args: (T, T, T, T, T, T, T)) -> Args {
        Args::SevenArgs((
            format!("{}", args.0),
            format!("{}", args.1),
            format!("{}", args.2),
            format!("{}", args.3),
            format!("{}", args.4),
            format!("{}", args.5),
            format!("{}", args.6),
        ))
    }
}

impl<T> From<(T, T, T, T, T, T, T, T)> for Args
where
    T: std::fmt::Display + std::marker::Sized,
{
    fn from(args: (T, T, T, T, T, T, T, T)) -> Args {
        Args::EightArgs((
            format!("{}", args.0),
            format!("{}", args.1),
            format!("{}", args.2),
            format!("{}", args.3),
            format!("{}", args.4),
            format!("{}", args.5),
            format!("{}", args.6),
            format!("{}", args.7),
        ))
    }
}

impl<T> From<(T, T, T, T, T, T, T, T, T)> for Args
where
    T: std::fmt::Display + std::marker::Sized,
{
    fn from(args: (T, T, T, T, T, T, T, T, T)) -> Args {
        Args::NineArgs((
            format!("{}", args.0),
            format!("{}", args.1),
            format!("{}", args.2),
            format!("{}", args.3),
            format!("{}", args.4),
            format!("{}", args.5),
            format!("{}", args.6),
            format!("{}", args.7),
            format!("{}", args.8),
        ))
    }
}

impl<T> From<(T, T, T, T, T, T, T, T, T, T)> for Args
where
    T: std::fmt::Display + std::marker::Sized,
{
    fn from(args: (T, T, T, T, T, T, T, T, T, T)) -> Args {
        Args::TenArgs((
            format!("{}", args.0),
            format!("{}", args.1),
            format!("{}", args.2),
            format!("{}", args.3),
            format!("{}", args.4),
            format!("{}", args.5),
            format!("{}", args.6),
            format!("{}", args.7),
            format!("{}", args.8),
            format!("{}", args.9),
        ))
    }
}
