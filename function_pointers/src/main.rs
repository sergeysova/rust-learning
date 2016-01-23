

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
}
