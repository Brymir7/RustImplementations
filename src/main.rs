use merge::test_merge_two_lists;
use trie::test_trie;
use twosum::test_two_sum;

pub mod merge;
pub mod twosum;
pub mod trie;
fn main() {
    test_merge_two_lists();
    test_trie();
    test_two_sum();
}