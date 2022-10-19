use std::collections::HashMap;

fn k_weakest_rows(mat: Vec<Vec<i32>>, k:i32) -> Vec<i32> {
  #[derive(Debug)]
  struct Pair {
    val: i32,
    idx: usize
  }
  let mut v:Vec<Pair> = vec![];
  for i in 0..mat.len() {
    v.push(Pair{val:0, idx:i});
    for j in 0..mat[i].len() {
      if mat[i][j] == 1 {
        v[i].val += 1;
      } else {
        break;
      }
    }
  }
  v.sort_by(|a,b| a.val.cmp(&b.val));
  let mut result: Vec<i32> = vec![];
  for i in 0..k {
    result.push(v[i as usize].idx as i32);
  }
  return result;
}

fn main() {
  let mut m1:Vec<Vec<i32>> = vec![];
  m1.push(vec![1,1,0,0,0]);
  m1.push(vec![1,1,1,1,0]);
  m1.push(vec![1,0,0,0,0]);
  m1.push(vec![1,1,0,0,0]);
  m1.push(vec![1,1,1,1,1]);

  let mut m2: Vec<Vec<i32>> = vec![];
  m2.push(vec![1,0,0,0]);
  m2.push(vec![1,1,1,1]);
  m2.push(vec![1,0,0,0]);
  m2.push(vec![1,0,0,0]);

  let result1 = k_weakest_rows(m1, 3);
  let result2 = k_weakest_rows(m2, 2);
  println!("Result1: {:?}", result1);
  println!("Result2: {:?}", result2);
}
