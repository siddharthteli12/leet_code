use std::cmp::Ordering;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (None, None) => None,
        (None, Some(nodelist2)) => Some(nodelist2),
        (Some(nodelist1), None) => Some(nodelist1),
        (Some(nodelist1), Some(nodelist2)) => match nodelist1.val.cmp(&nodelist2.val) {
            Ordering::Greater | Ordering::Equal => Some(Box::new(ListNode {
                val: nodelist2.val,
                next: merge_two_lists(Some(nodelist1), nodelist2.next),
            })),
            Ordering::Less => Some(Box::new(ListNode {
                val: nodelist1.val,
                next: merge_two_lists(nodelist1.next, Some(nodelist2)),
            })),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_list() {
        assert_eq!(merge_two_lists(None, None), None);
    }
}
