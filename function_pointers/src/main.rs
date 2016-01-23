

fn callback(cb: fn(i32, i32) -> i32) -> i32 {
  cb(12, 24)
}


fn main() {
  fn plus(a: i32, b: i32) -> i32 {
    a + b
  };

  let f = plus;
  println!("{}", f(12, -55));

  let var = callback(plus);
  println!("{}", var);

  arrays();
  tuples();
}

fn arrays() {
  let _a = [1, 2, 3]; // immutable array of [i32; 3]
  let mut _b = [1, 2, 3]; // mutable array of [i32; 3]

  let c = [0; 20]; // initialize 20 elements with 0

  let _middle = &c[5..15]; // get the middle of array
}


///
/// Simple tuple example
///
/// ```rust
///   let tuple = (1, true, "hello");
/// ```
///
fn tuples() {
  let a = (1, "hello");
  let b: (i32, &str) = a;

  let (x, y) =  b;

  println!("first in tuple: {}, second: {}", x, y);
  println!("show tuple: {{{}, {}}}", b.0, b.1);
}

