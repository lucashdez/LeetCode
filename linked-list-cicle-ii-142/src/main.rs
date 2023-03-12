use std::collections::LinkedList;

// fn detect_cycle(head:) -> ...{}
fn detect_cycle(head: LinkedList<i32>) -> i32 {}

fn main() {
    let mut llist: LinkedList<i32> = LinkedList::new();
    let result = detect_cycle(llist);
    println!("Result: {}", result)
}

#[test]
fn ex1() {
    // 3 -> 2 -> 0 -> -4
    //      |          |
    //      |----------|
    // idx_res = 1
}

#[test]
fn ex2() {
    // 1 -> 0
    // |    |
    // |----|
    // idx_res = 0
}

#[test]
fn ex3() {
    // 1
    // No cycle
}
