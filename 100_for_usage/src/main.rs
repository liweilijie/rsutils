fn main() {
    for_loop();
    println!("Hello, world!");
}

fn for_loop() {
    // for in example

    // for n in 1..=11 { // 闭合到11
    for n in 1..11 { // 1到10
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n)
        }
    }


}