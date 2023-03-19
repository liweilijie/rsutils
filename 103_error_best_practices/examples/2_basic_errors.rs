use std::error::Error;
use std::fmt::{Display, Formatter};

fn ok_dyn_error() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[derive(Debug)]
struct MyInnerError(String);

impl Display for MyInnerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn err_my_inner_error() -> Result<(), MyInnerError> {
    Err(MyInnerError("some inner reason".to_string()))
}

impl Error for MyInnerError {}

#[derive(Debug)]
enum MyError {
    FromInner(MyInnerError),
    Other(String)
}

fn string_error_my_error() -> Result<(), MyError> {
    Err(MyError::Other("xxx".to_string()))
}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
       write!(f, "{:?}", self)
    }
}

fn my_inner_error_my_error() -> Result<(), MyError> {
    Err(MyError::FromInner(MyInnerError(
        "some inner reason".to_string()
    )))
}

impl Error for MyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self  {
            MyError::FromInner(inner) => Some(inner),
            MyError::Other(_) => None,
        }
    }
}

fn multiple_error_types_return() -> Result<(), Box<dyn Error>> {
    my_inner_error_my_error()?;
    err_my_inner_error()?;
    let _: i32 = "aaa".parse()?;
    ok_dyn_error()?;

    Ok(())
}

fn main() {
    println!("Ok {:?}", ok_dyn_error());

    println!("My inner error {:?}", err_my_inner_error());
    println!(
        "My inner error unwraped {}",
        err_my_inner_error().expect_err("Not an error")
    );

    println!("My error with inner error {:?}", my_inner_error_my_error());

    println!("My error with string error {:?}", string_error_my_error());

    let my_inner_wrapped = my_inner_error_my_error().expect_err("Not an error");
    println!(
        "My error with inner error err unwrapped {}",
        my_inner_wrapped
    );
    println!("My error with inner source {:?}", my_inner_wrapped.source());

    println!("Multiple errors {:?}", multiple_error_types_return());
}