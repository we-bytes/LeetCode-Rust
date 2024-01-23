pub struct Solution {}

use crate::util::linked_list::{to_list, ListNode};
use std::cmp::{Ord, Ordering, PartialEq};
use std::collections::BinaryHeap;

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // 默认是最大堆，这里颠倒顺序，实现最小堆
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        }
        let mut ans = Box::new(ListNode::new(0));
        let mut ptr = &mut ans;
        let mut heap = BinaryHeap::new();
        // 把第一列的元素放到堆里
        for node in lists {
            if let Some(n) = node {
                heap.push(n);
            }
        }

        // 弹出最小的, 然后把它剩下的再加入到堆中
        while let Some(mut node) = heap.pop() {
            if let Some(next) = node.next.take() {
                heap.push(next)
            }
            ptr.next = Some(node);
            ptr = ptr.next.as_mut().unwrap();
        }

        ans.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s0023() {
        assert_eq!(
            Solution::merge_k_lists(vec![
                to_list(vec![1, 4, 5]),
                to_list(vec![1, 3, 4]),
                to_list(vec![2, 6]),
            ]),
            to_list(vec![1, 1, 2, 3, 4, 4, 5, 6])
        );
        assert_eq!(Solution::merge_k_lists(vec![]), None);
    }
}
