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

fn result_operations() {
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
}

fn main() -> Result<(), String> {
    empty_result();
    Ok(())
}