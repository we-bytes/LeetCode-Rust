pub struct Solution {}

use crate::util::linked_list::{to_list, ListNode};
impl Solution {
    // 快慢指针
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let (mut fast, mut slow) = (&head, &head);
        let mut result = None;
        let mut tail = &mut result;
        // 快指针先走n步
        for _ in 0..n - 1 {
            fast = &fast.as_ref().unwrap().next;
        }
        // 快慢指针一起走, 快指针走到尾, 慢指针刚好到n节点处
        while fast.as_ref().unwrap().next.is_some() {
            *tail = Some(Box::new(ListNode::new(slow.as_ref().unwrap().val)));
            tail = &mut tail.as_mut().unwrap().next;
            fast = &fast.as_ref().unwrap().next;
            slow = &slow.as_ref().unwrap().next;
        }
        *tail = slow.as_ref().unwrap().next.to_owned();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s0019() {
        assert_eq!(
            Solution::remove_nth_from_end(to_list(vec![1, 2, 3, 4, 5]), 2),
            to_list(vec![1, 2, 3, 5])
        );
        assert_eq!(Solution::remove_nth_from_end(to_list(vec![1]), 1), None);
    }
}
