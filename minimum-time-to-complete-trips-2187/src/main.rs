use colored::Colorize;

fn is_safe(time: &Vec<i32>, total: &i64) -> i64 {
    let mut res: i64 = 0;
    for i in 0..time.len() {
        res += (total / time[i] as i64) as i64
    }
    res
}

fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
    let mut low: i64 = 1;
    let mut _high: i64 = 1e14 as i64;
    while low < _high {
        let mid: i64 = low + (_high - low) / 2;
        if is_safe(&time, &mid) >= total_trips as i64 {
            _high = mid;
        } else {
            low = mid + 1;
        }
    }
    return low;
}

fn main() {
    let time = vec![1, 2, 3];
    let total_trips = 5;
    let result = minimum_time(time, total_trips);
    println!("Result: {}", result.to_string().green());
}

#[test]
fn test_1() {
    let time = vec![1, 2, 3];
    let total_trips = 5;
    let result = minimum_time(time, total_trips);
    assert_eq!(result, 3)
}

#[test]
fn test_2() {
    let time = vec![2];
    let total_trips = 1;
    let result = minimum_time(time, total_trips);
    assert_eq!(result, 2)
}
