// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut l1_crt = l1;
    let mut l2_crt = l2;
    let mut carry = 0;
    let mut head = Box::new(ListNode::new(0));
    let mut crt = &mut head;

    while l1_crt.is_some() || l2_crt.is_some() || carry != 0 {
        let mut sum = carry;

        if let Some(x) = l1_crt {
            sum += x.val;
            l1_crt = x.next;
        }

        if let Some(x) = l2_crt {
            sum += x.val;
            l2_crt = x.next;
        }

        carry = sum / 10;

        crt.next = Some(Box::new(ListNode::new(sum % 10)));
        crt = crt.next.as_mut().unwrap();
    }

    head.next
}

#[allow(dead_code)]
fn main () {

}
