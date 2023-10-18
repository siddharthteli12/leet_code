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

pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }
    let mut list = vec![];

    let mut pointer = &head;

    while pointer.is_some() {
        let val = pointer.as_ref().unwrap().val;
        list.push(val);
        pointer = &pointer.as_ref().unwrap().next;
    }

    head = Some(Box::new(ListNode {
        val: list.pop().unwrap(),
        next: None,
    }));
    let mut pointer = match head {
        Some(ref mut val) => &mut val.next,
        None => unreachable!(),
    };
    for value in list.iter().rev() {
        *pointer = Some(Box::new(ListNode {
            val: *value,
            next: None,
        }));
        pointer = match pointer {
            Some(val) => &mut val.next,
            None => unreachable!(),
        };
    }

    head
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_linked_list() {
        let link_list = None;
        assert_eq!(reverse_list(link_list), None);
    }

    #[test]
    fn single_element_list() {
        let link_list = Box::new(ListNode {
            val: 100,
            next: None,
        });
        assert_eq!(reverse_list(Some(link_list.clone())), Some(link_list));
    }

    #[test]
    fn multielement_list() {
        let link_list = Box::new(ListNode {
            val: 100,
            next: Some(Box::new(ListNode {
                val: 20,
                next: None,
            })),
        });
        assert_eq!(
            reverse_list(Some(link_list.clone())),
            Some(Box::new(ListNode {
                val: 20,
                next: Some(Box::new(ListNode {
                    val: 100,
                    next: None,
                })),
            }))
        );
    }
}
