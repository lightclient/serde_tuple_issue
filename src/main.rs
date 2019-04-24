use serde_tuple::*;

#[derive(Deserialize_tuple)]
struct A {
    _a: i32,
}

fn main() {
    println!("Hello, world!");
}
