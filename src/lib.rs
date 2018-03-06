#[no_mangle]
pub extern "C" fn run(num: usize) -> usize {
  return (0..100).fold(0, |acc, _| acc + f(num));
}

#[no_mangle]
pub extern "C" fn f(limit: usize) -> usize {
  let mut cache: Vec<usize> = vec![0; limit];

  let mut max = 0;
  let mut answer = 1;
  for i in 1..limit {
    let mut n = i;
    let mut cnt = 0;
    while n != 1 {
      cnt += 1;
      n = if n % 2 == 0 { n >> 1 } else { 3 * n + 1 };
      if n < i {
        cnt += cache[n];
        break;
      }
    }
    cache[i] = cnt;
    if cnt > max {
      max = cnt;
      answer = i
    }
  }
  answer
}

#[no_mangle]
pub extern "C" fn solve(n: usize) -> usize {
  let sum = (1..n + 1).fold(0, |acc, x| acc + x);
  let sum_square = (1..n + 1).fold(0, |acc, x| acc + x * x);
  sum * sum - sum_square
}
