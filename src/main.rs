use merge::test_merge_two_lists;
use parantheses::test_parantheses_checker;
use trie::test_trie;
use twosum::test_two_sum;

pub mod parantheses;
pub mod merge;
pub mod twosum;
pub mod trie;
fn main() {
    test_merge_two_lists();
    test_trie();
    test_two_sum();
    test_parantheses_checker();
}