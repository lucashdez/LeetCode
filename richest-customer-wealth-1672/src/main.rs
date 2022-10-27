fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
  let mut result: i32 = 0;
  let mut row_sum: i32 = 0;

  for i in 0..accounts.len() {
    row_sum = 0;
    for j in 0..accounts[i].len() {
      row_sum += accounts[i][j];
    }
    if row_sum >= result {
      result = row_sum;
    }
  }
  return result;
}

fn main() {
  let v: Vec<Vec<i32>> = vec![
    vec![2,8,7],
    vec![7,1,3],
    vec![1,9,5]
  ];
  println!("Input {:?}\nResult: {}", v, maximum_wealth(v.clone()));
}
