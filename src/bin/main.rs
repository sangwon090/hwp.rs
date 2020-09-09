#[macro_use]
extern crate nom;

use hwp::HwpDocument;

fn main() {
    let document: HwpDocument = HwpDocument::open("./test.hwp").unwrap();

    println!("Hello, World!");
}