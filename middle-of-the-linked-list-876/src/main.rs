#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
  val: i32,
  next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  let mut dummy = &head;
  let mut count = 0;
  while dummy.is_some() {
    dummy = &dummy.as_ref().unwrap().next;
    count += 1;
  }
  let middle = count/2; 
  dummy = &head;
  for _ in 0..middle {
    dummy = &dummy.as_ref().unwrap().next;
  }
  return dummy.clone();
}

fn main() {
  let i6 = Some(Box::new(ListNode::new(6)));
  let mut i5 = Some(Box::new(ListNode::new(5)));
  i5.as_mut().unwrap().next = i6;
  let mut i4 = Some(Box::new(ListNode::new(4)));
  i4.as_mut().unwrap().next = i5;
  let mut i3 = Some(Box::new(ListNode::new(3)));
  i3.as_mut().unwrap().next = i4;
  let mut i2 = Some(Box::new(ListNode::new(2)));
  i2.as_mut().unwrap().next = i3;
  let mut ll = Some(Box::new(ListNode::new(1)));
  ll.as_mut().unwrap().next = i2;
  let result = middle_node(ll);
  print!("Result: {}", result.unwrap().val);
}
