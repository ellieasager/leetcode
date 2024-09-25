use super::{ListNode, to_list, Solution};

impl Solution {

    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        
        if list1.is_none() { return list2 }
        if list2.is_none() { return list1 }

        let mut result = Some(Box::new(ListNode { val: 0, next: None }));
        let mut last_node_appended = &mut result;
        let (mut l1, mut l2) = (list1, list2);

        while l1.is_some() || l2.is_some() {
          if l1.is_none() { 
            last_node_appended.as_mut().unwrap().next = l2;
            break
          }
          if l2.is_none() { 
            last_node_appended.as_mut().unwrap().next = l1;
            break
          }

          let node1 = l1.as_ref().unwrap();
          let node2 = l2.as_ref().unwrap();

          let next_node_to_append = if node1.val > node2.val { 
            let (head, remaining_list) = Solution::take_head(l2);
            l2 = remaining_list;
            head
          } else { 
            let (head, remaining_list) = Solution::take_head(l1);
            l1 = remaining_list;
            head
          };
          last_node_appended.as_mut().unwrap().next = next_node_to_append;
          last_node_appended = &mut last_node_appended.as_mut().unwrap().next;
        }
        result.unwrap().next
    }

    #[inline]
    fn take_head(mut list: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let tail = list.as_mut().unwrap().next.take();
        let head = list.take();
        (head, tail)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    mod tests {
        use super::*;
    
        #[test]
        fn test_21() {
            assert_eq!(
                Solution::merge_two_lists(to_list(vec![1, 2, 4]), to_list(vec![1, 3, 4])),
                to_list(vec![1, 1, 2, 3, 4, 4])
            );
        }
    }
}