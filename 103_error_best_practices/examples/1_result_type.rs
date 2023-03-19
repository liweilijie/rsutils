fn empty_result() {
    let empty_result: Result<(), ()> = Ok(());
    println!("Empty result ok - {:?}", empty_result);

    let empty_result: Result<(), ()> = Err(());
    println!("Empty result err - {:?}", empty_result);
}

fn ok_int_string() -> Result<i32, String> {
    Ok(3)
}

fn err_int_string() -> Result<i32, String> {
    Err("Error".to_string())
}

fn return_operations() {
    // Unwrapping
    println!("unwrap ok {}", ok_int_string().unwrap());
    println!("Expect ok {}", ok_int_string().expect("Expected ok"));

    println!(
        "Expect err {}",
        err_int_string().expect_err("Expected error")
    );

    // Basic checks
    println!("Result is ok {}", ok_int_string().is_ok());
    println!("Result is err {}", ok_int_string().is_err());

    // Transform to option
    println!("Result ok {:?}", ok_int_string().ok());
    println!("Result err {:?}", ok_int_string().err());

    // Operations
    println!("Result map {:?}", ok_int_string().map(|v| v as f32 * 3.0));
    println!(
        "Result map err {:?}",
        ok_int_string().map_err(|v| MyComplexError(v))
    );
    println!(
        "Result map or else {:?}",
        ok_int_string().map_or_else(|v| 6., |v| 3.0 * v as f32)
    );
    println!(
        "Result map or {:?}",
        ok_int_string().map_or(6.0, |v| 3.0 * v as f32)
    );

    println!(
        "Result and_then {:?}",
        ok_int_string().and_then(|v| Ok(v * 5))
    );

    println!(
        "Result and chain {:?}",
        ok_int_string().and(ok_int_string()).and(err_int_string())
    );
}

fn return_error_string() -> Result<(), String> {
    Err("string error".to_string())
}

#[derive(Debug)]
struct MyErrorType;

fn return_my_error_type() -> Result<(), MyErrorType> {
    Err(MyErrorType)
}

fn simple_early_return() -> Result<i32, MyErrorType> {
    return_my_error_type()?;
    return Ok(20);
}

#[derive(Debug)]
struct MyComplexError(String);

impl From<MyErrorType> for MyComplexError {
    fn from(value: MyErrorType) -> Self {
        MyComplexError("From MyErrorType".to_string())
    }
}

impl From<String> for MyComplexError {
    fn from(value: String) -> Self {
       MyComplexError(format!("My complex error from string: {value}"))
    }
}

fn error_transformation_return() -> Result<i32, MyComplexError> {
    simple_early_return()?;
    return_error_string()?;

    Ok(42)
}

fn early_return() {
    println!("simple_early_return = {:?}", simple_early_return());
    println!(
        "error_transformation_return = {:?}",
        error_transformation_return()
    );
}

fn main() -> Result<(), String> {
    empty_result();
    return_operations();
    early_return();
    Ok(())
}