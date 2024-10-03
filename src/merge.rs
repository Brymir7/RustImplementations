use crate::merge;

#[derive(PartialEq, Eq, Clone, Debug)]
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
    fn new_chain(values: Vec<i32>) -> Self {
        let mut prehead: ListNode = ListNode::new(-1);
        let mut curr_node = &mut prehead;
        for (i, value) in values.iter().enumerate() {
            curr_node.val = *value;
            if i != values.len() - 1 {
                curr_node.next = Some(Box::new(ListNode::new(-1)));
                curr_node = curr_node.next.as_mut().unwrap();
            }
        }
        return prehead;
    }
    fn print(self) {
        let mut current = &self;
        print!("{}", current.val);
        while let Some(ref node) = current.next {
            print!("{}", node.val);
            current = node;
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

pub fn test_merge_two_lists() {
    let mut chain = ListNode::new_chain(vec![1, 2, 4]);
    let mut chain2 = ListNode::new_chain(vec![1, 3, 4]);
    println!("Trying to merge following");
    chain.clone().print();
    println!("");
    chain2.clone().print();
    println!("");
    println!("Result");
    let chain3 = merge_two_lists(Some(Box::new(chain)), Some(Box::new(chain2)));
    chain3.unwrap().print();
}
