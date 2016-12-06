
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
struct Base {
    value: u32,
}

impl Display for Base {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.value.to_string())
    }
}

fn main() {
    let b = Base { value: 290 };
    println!("result: {}", b);
    println!("debug: {:?}", b);
}

