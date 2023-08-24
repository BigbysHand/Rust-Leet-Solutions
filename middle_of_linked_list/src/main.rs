// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  //#[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}


pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut pointer: Option<Box<ListNode>> = head.clone();
    let mut length: i32 = 0;
    while head != None{
        length += 1;
        head = head.unwrap().next;
    }
    for i in 0..(length /2){
        pointer = pointer.unwrap().next;
    }
    return pointer;
}


fn main() {
}
