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

fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
  let dummy = &head;
  let _count = |mut dummy: &Option<Box<ListNode>>| -> i32 {
    let mut result = 0;
    while dummy.is_some() {
      dummy = &dummy.as_ref().unwrap().next;
      result += 1;
    }
    return result;
  };
  let size = _count(dummy);

  let mid: i32 = size/2;
  let mut first = &mut head;
  let mut second: Option<Box<ListNode>>;
  for _ in 1..mid {
      first = &mut first.as_mut().unwrap().next;
  }

  if size % 2 == 0 {
    second = first.as_mut().unwrap().next.take();
  } else {
    // NOTE: Get rid of the mid elem;
    let tmp = first.as_mut().unwrap().next.take();
    second = tmp.unwrap().next.take();
  }

  // NOTE: INVERTIR
  let mut prev: Option<Box<ListNode>> = None;
  let mut curr = head;
  let mut next: Option<Box<ListNode>>;
  while curr.is_some() {
    let mut node = curr.unwrap();
    next = node.next.take();
    node.next = prev;
    prev = Some(node);
    curr = next;
  }
  
  // ANALIZAMOS
  while prev.is_some() && second.is_some() {
    let node1 = prev.unwrap();
    let node2 = second.unwrap();
    if node1.val != node2.val {
      return false;
    } else {
      prev = node1.next;
      second = node2.next;
    }
  }
  return true;
}



fn main() {
  let i4 = ListNode::new(1);
  let mut i3 = ListNode::new(2);
  i3.next = Option::Some(Box::new(i4));
  let mut i2 = ListNode::new(2);
  i2.next = Option::Some(Box::new(i3));
  let mut ll = Option::Some(Box::new(ListNode::new(1)));
  ll.as_mut().unwrap().next = Option::Some(Box::new(i2));
  print!("Is palindrome: {}",is_palindrome(ll));


}
