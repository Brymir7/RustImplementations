pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}
pub fn merge_two_lists(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>
) -> Option<Box<ListNode>> {
    let mut prehead = ListNode::new(-1);
    let mut cur_node = &mut prehead;

    while let (Some(node1), Some(node2)) = (&list1, &list2) {
        if node1.val < node2.val {
            cur_node.next = list1.take();
            cur_node = cur_node.next.as_mut().unwrap();
            list1 = cur_node.next.take();
        } else {
            cur_node.next = list2.take();
            cur_node = cur_node.next.as_mut().unwrap();
            list2 = cur_node.next.take();
        }
    }
    cur_node.next = list1.or(list2);
    prehead.next
}
