
use std::thread;

#[no_mangle]
pub extern fn process() {
  let handles: Vec<_> = (0..10).map(|_| {
    thread::spawn(|| {
      let mut x = 0;
      for _ in 0..5_000_00 {
        x += 1
      }
      x
    })
  }).collect();

  for h in handles {
    println!("Thead finished with count={}",
              h.join().map_err(|_| "Could not join a thread!").unwrap());
  }
}

#[no_mangle]
pub extern fn demo(name: &str) {
  println!("{}, Hello world!", name.to_string());
}
