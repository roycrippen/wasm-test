#[no_mangle]
pub extern "C" fn f(n: usize) -> usize {
  let mut longest: usize = 0;
  let mut terms: usize = 0;
  let mut j: usize;

  for i in 1..n {
    j = i as usize;
    let mut this_terms = 1;

    while j != 1 {
      this_terms += 1;

      if this_terms > terms {
        terms = this_terms;
        longest = i;
      }

      if j % 2 == 0 {
        j = j / 2;
      } else {
        j = 3 * j + 1;
      }
    }
    // println!("{}", i);
  }
  return longest;
}

#[no_mangle]
pub extern "C" fn run(num: usize) -> usize {
  return (0..100).fold(0, |acc, _| acc + f2(num));
}

#[no_mangle]
pub extern "C" fn f2(limit: usize) -> usize {
  let mut cache: Vec<usize> = vec![0; limit];

  let mut max = 0;
  let mut answer = 1;
  for (i, _) in cache.clone().iter().enumerate().skip(1) {
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
pub extern "C" fn solve(n: i64) -> i64 {
  let sum = (1..n + 1).fold(0, |acc, x| acc + x);
  let sum_square = (1..n + 1).fold(0, |acc, x| acc + x * x);
  sum * sum - sum_square
}
