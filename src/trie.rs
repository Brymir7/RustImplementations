const ALPHABET_SIZE: u8 = u8::MAX;
struct TrieNode {
    curr_letter: u8,
    is_end_of_word: bool,
    children: Option<Box<[TrieNode; ALPHABET_SIZE as usize]>>,
}
impl TrieNode {
    fn root_node() -> Self {
        return TrieNode {
            curr_letter: 0,
            children: Some(Box::new(core::array::from_fn(|_| TrieNode::placeholder()))),
            is_end_of_word: false,
        };
    }
    fn placeholder() -> Self {
        TrieNode {
            curr_letter: 0,
            children: None,
            is_end_of_word: false,
        }
    }
    fn initialize_children(&mut self) {
        self.children = Some(Box::new(core::array::from_fn(|_| TrieNode::placeholder())));
    }
}
struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::root_node(),
        }
    }

    fn insert(&mut self, word: String) {
        let mut curr_layer = &mut self.root;
        for (i, char) in word.chars().enumerate() {
            debug_assert!(char.is_ascii());
            let int_char = char as u8;
            if curr_layer.children.is_none() {
                curr_layer.initialize_children();
            }
            curr_layer = &mut curr_layer.children.as_mut().unwrap()[int_char as usize];
            curr_layer.curr_letter = int_char;
            if i == word.len() - 1 {
                curr_layer.is_end_of_word = true;
            }
        }
    }

    fn search(&self, word: String) -> bool {
        let mut curr_layer = &self.root;
        for char in word.chars() {
            debug_assert!(char.is_ascii());
            let int_char = char as u8;
            if let Some(children) = &curr_layer.children {
                let bucket = &children[int_char as usize];
                if int_char > 0 && bucket.curr_letter != int_char {
                    return false;
                }
                curr_layer = bucket;
            } else {
                return false;
            }
        }
        return curr_layer.is_end_of_word;
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut curr_layer = &self.root;
        for char in prefix.chars() {
            debug_assert!(char.is_ascii());
            let int_char = char as u8;
            if let Some(children) = &curr_layer.children {
                let bucket = &children[int_char as usize];
                if int_char > 0 && bucket.curr_letter != int_char {
                    return false;
                }
                curr_layer = bucket;
            } else {
                return false;
            }
        }
        return true;
    }
}

pub fn test_trie() {
    let mut trie = Trie::new();
    trie.insert("apple".into());
    debug_assert!(trie.search("apple".into())); // return True
    debug_assert!(trie.search("app".into()) == false); // return False
    debug_assert!(trie.starts_with("app".into())); // return True
    trie.insert("app".into());
    debug_assert!(trie.search("app".into())); // return True
}
