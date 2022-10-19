// I, V, X, L, C, D, M
fn roman_to_int(s: String) -> i32 {
  enum Roman {
    I, V, X, L, C, D, M
  }
  let mut result: i32 = 0;
  let values: Vec<i32> = vec![1, 5, 10, 50, 100, 500, 1000];
  let chars: Vec<char> = s.chars().collect();
  for c in 0..chars.len() {
    match chars[c] {
      'I' => {
        let val = values[Roman::I as usize];
        if c+1 != chars.len() && (chars[c+1] == 'V' || chars[c+1] == 'X') {
          result -= val;
        } else {
          result += val;
        }
      },
      'V' => {result += values[Roman::V as usize]},
      'X' => {
        let val = values[Roman::X as usize];
        if c+1 != chars.len() && (chars[c+1] == 'L' || chars[c+1] == 'C') {
          result -= val;
        } else {
          result += val;
        }
      },
      'L' => {result += values[Roman::L as usize]},
      'C' => {
        let val = values[Roman::C as usize];
        if c+1 != chars.len() && (chars[c+1] == 'D' || chars[c+1] == 'M') {
          result -= val;
        } else {
          result += val;
        }
      },
      'D' => {result += values[Roman::D as usize]},
      'M' => {result += values[Roman::M as usize]},
      _ => {
        print!("None");
        panic!("Something happened with something");
      },
    }
  }
  return result;
}

fn main() {
  println!("III = {}", roman_to_int("III".to_string()));
  println!("LVIII = {}", roman_to_int("LVIII".to_string()));
  println!("MCMXCIV = {}", roman_to_int("MCMXCIV".to_string()));
  println!("MCDXCV = {}", roman_to_int("MCDXCV".to_string()));
}
