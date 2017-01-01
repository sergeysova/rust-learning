mod demo;
use demo::tra::Tra;

fn main() {
    let t = demo::Type::new(1, "foo");
    println!("{}", t.concat());
    println!(": {}", t.foo());

    println!("Hello, world!");
}
