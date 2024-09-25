pub mod s001_two_sum;
pub mod s009_palindrome_number;
pub mod s013_roman_to_int;
pub mod s014_longest_common_prefix;
pub mod s020_valid_parentheses;
pub mod s021_merge_two_lists;
pub mod s026_remove_duplicates;

pub struct Solution {
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
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

// helper function for test
pub fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &v in vec.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = current;
        current = Some(Box::new(node));
    }
    current
}