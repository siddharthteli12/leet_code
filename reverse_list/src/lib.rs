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

    let new_list = create_link_list(list);

    new_list
}

fn create_link_list(mut list: Vec<i32>) -> Option<Box<ListNode>> {
        match list.pop() {
            Some(val) => return Some(Box::new(ListNode { val, next: create_link_list(list) })),
            None => None
        }
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
