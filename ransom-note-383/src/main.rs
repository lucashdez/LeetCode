use std::collections::HashMap;

fn can_construct(ransom_note: String, magazine: String) -> bool {
  let mut hash: HashMap<char, i32> = HashMap::new();
  let mut result: bool = true;
  for c in magazine.chars() {
    if hash.contains_key(&c) {
      hash.entry(c)
        .and_modify(|v| *v += 1);
    } else {
      hash.insert(c, 1);
    }
  }
  for c in ransom_note.chars() {
    if hash.contains_key(&c) {
      hash.entry(c)
        .and_modify(|v| *v -= 1);
      let tmp = hash.get(&c).unwrap();
      if *tmp == -1 {
        result = false;
        break;
      }
    } else {
      result = false;
      break;
    }
  }
  return result;
}

fn main() {
  let p1 = can_construct("aa".to_string(), "ab".to_string());
  let p2 = can_construct("aa".to_string(), "aab".to_string());
  print!("Res1: {}\nRes2: {}", p1, p2);
}
