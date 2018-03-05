pub mod lib;

fn main() {
  assert!(lib::f(100) == lib::f(100));
  println!("res = {}", lib::run(10_000));
  // println!("p006 = {} == 25164150", lib::solve(100))
}
