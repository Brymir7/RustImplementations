const ALPHABET_SIZE: u8 = u8::MAX;
const PLACEHOLDER_ASCII: u8 = u8::MIN;
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
            curr_letter: PLACEHOLDER_ASCII,
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
#[derive(PartialEq, Eq)]
enum TrieSearchResult {
    Nothing,
    Prefix,
    WholeWord,
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

    fn bool_search(&self, word: String) -> bool {
        return self.search(word) == TrieSearchResult::WholeWord;
    }
    fn search(&self, word: String) -> TrieSearchResult {
        let mut curr_layer = &self.root;
        for char in word.chars() {
            debug_assert!(char.is_ascii());
            let int_char = char as u8;
            if let Some(children) = &curr_layer.children {
                let bucket = &children[int_char as usize];
                if int_char > PLACEHOLDER_ASCII && bucket.curr_letter != int_char {
                    return TrieSearchResult::Nothing;
                }
                curr_layer = bucket;
            } else {
                return TrieSearchResult::Nothing;
            }
        }
        match curr_layer.is_end_of_word {
            true => {
                return TrieSearchResult::WholeWord;
            }
            false => {
                return TrieSearchResult::Prefix;
            }
        };
    }
    fn starts_with(&self, prefix: String) -> bool {
        let res = self.search(prefix);
        match res {
            TrieSearchResult::Nothing => false,
            TrieSearchResult::Prefix => true,
            TrieSearchResult::WholeWord => true,
        }
    }
}

pub fn test_trie() {
    let mut trie = Trie::new();
    trie.insert("apple".into());
    debug_assert!(trie.bool_search("apple".into())); // return True
    debug_assert!(trie.bool_search("app".into()) == false); // return False
    debug_assert!(trie.starts_with("app".into())); // return True
    trie.insert("app".into());
    debug_assert!(trie.bool_search("app".into())); // return True
}
