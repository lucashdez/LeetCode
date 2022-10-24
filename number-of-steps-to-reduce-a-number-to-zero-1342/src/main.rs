fn number_of_steps(num: i32) -> i32 {
  let mut steps:i32 = 0;
  let mut n:i32 = num;
  loop {
    if n == 0 {
      break;
    }
    if n % 2 == 0 {
      n = n / 2;
    } else {
      n = n - 1;
    }
    steps += 1;
  }
  return steps;
}

fn main() {
  println!("14 - Result: {}", number_of_steps(14));
  println!("14 - Result: {}", number_of_steps(8));
}
