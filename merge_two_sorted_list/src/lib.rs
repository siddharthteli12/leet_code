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

    fn insert(mut self, val: i32) -> Self {
        match self.next {
            Some(node) => node.insert(val),
            None => {
                self.next = Some(Box::new(ListNode::new(val)));
                self
            }
        }
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

    #[test]
    fn test_with_merge_with_both_list() {
        let list1 = Box::new(ListNode::new(10));
        let list1 = Box::new(list1.insert(12));

        let list2 = Box::new(ListNode::new(101));
        let list2 = Box::new(list2.insert(212));

        let result_list = Box::new(ListNode {
            val: 10,
            next: Some(Box::new(ListNode {
                val: 12,
                next: Some(Box::new(ListNode {
                    val: 101,
                    next: Some(Box::new(ListNode {
                        val: 212,
                        next: None,
                    })),
                })),
            })),
        });
        assert_eq!(merge_two_lists(Some(list1), Some(list2)), Some(result_list));
    }
}
