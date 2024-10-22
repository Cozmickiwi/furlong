use std::time::Instant;

use furlong::*;

fn main() {
    let now = Instant::now();
    let obj = Fobj::parse_obj("./cube.obj", [0.0; 3], String::from("cube1"));
    let el = now.elapsed();
    println!("{:#?}", obj);
    println!();
    println!("{:?}", el);
}
