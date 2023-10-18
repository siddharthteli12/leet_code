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
pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }
    let mut head = head.unwrap();
    let mut result_list = Box::new(ListNode {
        val: head.val,
        next: None,
    });
    let mut pointer = &mut result_list.next;
    let mut pointer2 = &head.next;
    let mut value1 = head.val;
    while pointer2.is_some() {
        let mut value2 = pointer2.as_ref().unwrap().val;
        pointer2 = &pointer2.as_ref().unwrap().next;
        if value1 != value2 {
            *pointer = Some(Box::new(ListNode {
                val: value2,
                next: None,
            }));
            pointer = match pointer {
                Some(val) => &mut val.next,
                None => unreachable!(),
            };
            value1 = value2;
        }
    }

    Some(result_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_list() {
        let linked_list = None;
        assert_eq!(delete_duplicates(linked_list.clone()), None);
    }

    #[test]
    fn simple_list() {
        let linked_list = Some(Box::new(ListNode {
            val: 100,
            next: Some(Box::new(ListNode {
                val: 20,
                next: None,
            })),
        }));

        assert_eq!(delete_duplicates(linked_list.clone()), linked_list);
    }
}
