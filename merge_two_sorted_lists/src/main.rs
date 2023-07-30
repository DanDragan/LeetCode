// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}
 
pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match  (list1, list2) {
        (Some(l1), None) => return Some(l1),
        (None, Some(l2)) => return Some(l2),
        (None, None) => return None,
        (Some(l1), Some(l2)) => {
            if l1.val <= l2.val {
                return Some(Box::new(ListNode{ next: merge_two_lists(l1.next, Some(l2)), val: l1.val }));
            } else {
                return Some(Box::new(ListNode{ next: merge_two_lists(Some(l1), l2.next), val: l2.val }));
            }
        },
    }
}

#[allow(dead_code)]
fn main() {
}
