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
    fn new_chain(values: Vec<i32>) -> Box<ListNode> {
        let mut prehead = ListNode::new(-1);
        let mut curr_node = &mut prehead;

        for value in values {
            curr_node.next = Some(Box::new(ListNode::new(value)));
            curr_node = curr_node.next.as_mut().unwrap();
        }
        prehead.next.unwrap()
    }

    fn print(self) {
        let mut current = &self;
        print!("{}", current.val);
        while let Some(ref node) = current.next {
            print!("{}", node.val);
            current = node;
        }
    }
    fn to_vec(&self) -> Vec<i32> {
        let mut result = vec![self.val];
        let mut current = self;
        while let Some(ref node) = current.next {
            result.push(node.val);
            current = node;
        }
        result
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
    let chain = ListNode::new_chain(vec![1, 2, 4]);
    let chain2 = ListNode::new_chain(vec![1, 3, 4]);
    let merged_list = merge_two_lists(Some(chain), Some(chain2));
    let result = merged_list.unwrap().to_vec();
    assert_eq!(result, vec![1, 1, 2, 3, 4, 4]);
}
