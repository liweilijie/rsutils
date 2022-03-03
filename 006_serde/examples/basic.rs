use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 5, y: 20 };
    let serialized = serde_json::to_string(&p).unwrap();
    println!("serialized = {}", serialized);

    let dp: Point = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", dp);
}