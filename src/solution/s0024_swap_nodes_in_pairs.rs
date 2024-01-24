pub struct Solution {}

use crate::util::linked_list::{to_list, ListNode};
impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));
        let mut head = dummy_head.as_mut();

        loop {
            let mut left = head.as_mut().unwrap().next.take();
            if left.is_none() {
                break; // 如果第一个节点为空就跳出
            }
            let mut right = left.as_mut().unwrap().next.take();
            // handle the un-paired one, e.g. [1, 2, 3] -> [2, 1, 3], 3 is un-paired
            if right.is_none() {
                head.as_mut().unwrap().next = left;
                break; // 如果第二个节点为空就跳出
            }
            let mut next = right.as_mut().unwrap().next.take(); // 第三个节点
            left.as_mut().unwrap().next = next;
            right.as_mut().unwrap().next = left;
            head.as_mut().unwrap().next = right;
            // BEFORE: head -> left -> right -> next
            // AFTER: head -> right -> left -> next
            head = head.unwrap().next.as_mut().unwrap().next.as_mut();
        }
        dummy_head.unwrap().next
    }

    // 递归
    pub fn swap_pairs2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(mut node1) = head {
            match node1.next.take() {
                None => return Some(node1),
                Some(mut node2) => {
                    node2.next.insert(node1).next = Self::swap_pairs2(node2.next.take());
                    return Some(node2);
                }
            }
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s0024() {
        assert_eq!(
            Solution::swap_pairs2(to_list(vec![1, 2, 3, 4])),
            to_list(vec![2, 1, 4, 3])
        );
        assert_eq!(Solution::swap_pairs2(to_list(vec![])), to_list(vec![]));
        assert_eq!(
            Solution::swap_pairs2(to_list(vec![1, 2, 3])),
            to_list(vec![2, 1, 3])
        );
        assert_eq!(Solution::swap_pairs2(to_list(vec![1])), to_list(vec![1]));
    }
}
