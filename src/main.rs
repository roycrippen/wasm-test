pub mod lib;

fn main() {
  println!("p006 = {}", lib::solve(100));
  println!("res = {}", lib::run(100_000))
}
