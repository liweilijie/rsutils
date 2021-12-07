fn main() {
    let mut value = 1;
    let r = &mut value;
    let r2 = &r;
    println!("{}", r2);
    println!("Hello, world!");
}
