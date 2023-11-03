use std::slice::RSplit;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn merge_two_lists(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut result_list = None;
    let mut pointer = &mut result_list;
    while list1.is_some() && list2.is_some() {
        let val = if list1.as_ref().unwrap().val < list2.as_ref().unwrap().val {
            let value = list1.as_ref().unwrap().val;
            list1 = list1.unwrap().next;
            value
        } else {
            let value = list2.as_ref().unwrap().val;
            list2 = list2.unwrap().next;
            value
        };
        *pointer = Some(Box::new(ListNode { val, next: None }));
        pointer = &mut pointer.as_deref_mut().unwrap().next;
    }

    while list1.is_some() {
        let val = list1.as_ref().unwrap().val;
        list1 = list1.unwrap().next;
        *pointer = Some(Box::new(ListNode { val, next: None }));
        pointer = &mut pointer.as_deref_mut().unwrap().next;
    }

    while list2.is_some() {
        let val = list2.as_ref().unwrap().val;
        list2 = list2.unwrap().next;
        *pointer = Some(Box::new(ListNode { val, next: None }));
        pointer = &mut pointer.as_deref_mut().unwrap().next;
    }
    result_list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let list1 = Some(Box::new(ListNode {
            val: 12,
            next: Some(Box::new(ListNode {
                val: 20,
                next: None,
            })),
        }));

        let list2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 25,
                next: None,
            })),
        }));

        let result = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 12,
                next: Some(Box::new(ListNode {
                    val: 20,
                    next: Some(Box::new(ListNode {
                        val: 25,
                        next: None,
                    })),
                })),
            })),
        }));

        assert_eq!(merge_two_lists(list1, list2), result);
    }
}
