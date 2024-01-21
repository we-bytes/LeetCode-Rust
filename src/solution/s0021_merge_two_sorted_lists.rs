pub struct Solution {}

use crate::util::linked_list::{to_list, ListNode};
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (l, None) => l,
            (None, r) => r,
            (Some(mut l), Some(mut r)) => {
                if l.val > r.val {
                    r.next = Self::merge_two_lists(Some(l), r.next);
                    Some(r)
                } else {
                    l.next = Self::merge_two_lists(l.next, Some(r));
                    Some(l)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s0021() {
        assert_eq!(
            Solution::merge_two_lists(to_list(vec![1, 2, 4]), to_list(vec![1, 3, 4])),
            to_list(vec![1, 1, 2, 3, 4, 4])
        );
    }
}
