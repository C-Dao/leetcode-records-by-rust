/*
* @lc app=leetcode id=208 lang=rust
*
* [208] Implement Trie (Prefix Tree)
*
* https://leetcode.com/problems/implement-trie-prefix-tree/description/
*
* algorithms
* Medium (59.26%)
* Likes:    8731
* Dislikes: 106
* Total Accepted:    718K
* Total Submissions: 1.2M
* Testcase Example:  '["Trie","insert","search","search","startsWith","insert","search"]\n' +
 '[[],["apple"],["apple"],["app"],["app"],["app"],["app"]]'
*
* A trie (pronounced as "try") or prefix tree is a tree data structure used to
* efficiently store and retrieve keys in a dataset of strings. There are
* various applications of this data structure, such as autocomplete and
* spellchecker.
*
* Implement the Trie class:
*
*
* Trie() Initializes the trie object.
* void insert(String word) Inserts the string word into the trie.
* boolean search(String word) Returns true if the string word is in the trie
* (i.e., was inserted before), and false otherwise.
* boolean startsWith(String prefix) Returns true if there is a previously
* inserted string word that has the prefix prefix, and false otherwise.
*
*
*
* Example 1:
*
*
* Input
* ["Trie", "insert", "search", "search", "startsWith", "insert", "search"]
* [[], ["apple"], ["apple"], ["app"], ["app"], ["app"], ["app"]]
* Output
* [null, null, true, false, true, null, true]
*
* Explanation
* Trie trie = new Trie();
* trie.insert("apple");
* trie.search("apple");   // return True
* trie.search("app");     // return False
* trie.startsWith("app"); // return True
* trie.insert("app");
* trie.search("app");     // return True
*
*
*
* Constraints:
*
*
* 1 <= word.length, prefix.length <= 2000
* word and prefix consist only of lowercase English letters.
* At most 3 * 10^4 calls in total will be made to insert, search, and
* startsWith.
*
*
*/

// @lc code=start
#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_word: bool,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, word: String) {
        let mut node = self;

        for u in word.as_bytes() {
            let index = (*u - b'a') as usize;
            node = node.children[index].get_or_insert_with(|| Box::new(Trie::new()));
        }

        node.is_word = true;
    }

    fn search(&self, word: String) -> bool {
        let mut node = self;

        for u in word.as_bytes() {
            let index = (*u - b'a') as usize;

            if node.children[index].is_none() {
                return false;
            }
            node = node.children[index].as_ref().unwrap();
        }

        return node.is_word;
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut node = self;

        for u in prefix.as_bytes() {
            let index = (*u - b'a') as usize;

            if node.children[index].is_none() {
                return false;
            }
            node = node.children[index].as_ref().unwrap();
        }

        true
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */
// @lc code=end

fn main() {
    let mut trie = Trie::new();
    trie.insert("apple".to_string());
    assert_eq!(trie.search("apple".to_string()), true);
    assert_eq!(trie.search("app".to_string()), false);
    assert_eq!(trie.starts_with("app".to_string()), true);
    trie.insert("app".to_string());
    assert_eq!(trie.search("app".to_string()), true);
}
