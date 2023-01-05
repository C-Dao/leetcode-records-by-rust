/*
* @lc app=leetcode id=676 lang=rust
*
* [676] Implement Magic Dictionary
*
* https://leetcode.com/problems/implement-magic-dictionary/description/
*
* algorithms
* Medium (56.68%)
* Likes:    1195
* Dislikes: 192
* Total Accepted:    69.6K
* Total Submissions: 122.3K
* Testcase Example:  '["MagicDictionary", "buildDict", "search", "search", "search", "search"]\n' +
 '[[], [["hello","leetcode"]], ["hello"], ["hhllo"], ["hell"], ["leetcoded"]]'
*
* Design a data structure that is initialized with a list of different words.
* Provided a string, you should determine if you can change exactly one
* character in this string to match any word in the data structure.
*
* Implement the MagicDictionary class:
*
*
* MagicDictionary() Initializes the object.
* void buildDict(String[] dictionary) Sets the data structure with an array of
* distinct strings dictionary.
* bool search(String searchWord) Returns true if you can change exactly one
* character in searchWord to match any string in the data structure, otherwise
* returns false.
*
*
*
* Example 1:
*
*
* Input
* ["MagicDictionary", "buildDict", "search", "search", "search", "search"]
* [[], [["hello", "leetcode"]], ["hello"], ["hhllo"], ["hell"], ["leetcoded"]]
* Output
* [null, null, false, true, false, false]
*
* Explanation
* MagicDictionary magicDictionary = new MagicDictionary();
* magicDictionary.buildDict(["hello", "leetcode"]);
* magicDictionary.search("hello"); // return False
* magicDictionary.search("hhllo"); // We can change the second 'h' to 'e' to
* match "hello" so we return True
* magicDictionary.search("hell"); // return False
* magicDictionary.search("leetcoded"); // return False
*
*
*
* Constraints:
*
*
* 1 <= dictionary.length <= 100
* 1 <= dictionary[i].length <= 100
* dictionary[i] consists of only lower-case English letters.
* All the strings in dictionary are distinct.
* 1 <= searchWord.length <= 100
* searchWord consists of only lower-case English letters.
* buildDict will be called only once before search.
* At most 100 calls will be made to search.
*
*
*/

// @lc code=start
#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_word: bool,
}

impl Trie {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, word: &str) {
        let mut s = self;

        for u in word.as_bytes() {
            s = s.children[*u as usize - 'a' as usize].get_or_insert_with(|| Box::new(Trie::new()));
        }

        s.is_word = true;
    }

    pub fn dfs(&self, word: &str, index: usize, edit_count: i32) -> bool {
        let word_bytes = word.as_bytes();

        if self.is_word && index == word.len() && edit_count == 1 {
            return true;
        }

        if index < word.len() && edit_count <= 1 {
            for i in 0..26 {
                let next_edit_count = if i == word_bytes[index] as usize - 'a' as usize {
                    edit_count
                } else {
                    edit_count + 1
                };

                if let Some(ref child) = self.children[i] {
                    if child.dfs(word, index + 1, next_edit_count) {
                        return true;
                    }
                }
            }
        }

        false
    }
}
struct MagicDictionary {
    trie: Trie,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MagicDictionary {
    fn new() -> Self {
        Self { trie: Trie::new() }
    }

    fn build_dict(&mut self, dictionary: Vec<String>) {
        for word in dictionary.iter() {
            self.trie.insert(word);
        }
    }

    fn search(&self, search_word: String) -> bool {
        self.trie.dfs(&search_word, 0, 0)
    }
}

/**
 * Your MagicDictionary object will be instantiated and called as such:
 * let obj = MagicDictionary::new();
 * obj.build_dict(dictionary);
 * let ret_2: bool = obj.search(searchWord);
 */
// @lc code=end

fn main() {
    let mut magic_dictionary = MagicDictionary::new();
    magic_dictionary.build_dict(vec!["hello".to_string(), "leetcode".to_string()]);
    assert_eq!(magic_dictionary.search("hello".to_string()), false);
    assert_eq!(magic_dictionary.search("hhllo".to_string()), true);
    assert_eq!(magic_dictionary.search("hell".to_string()), false);
    assert_eq!(magic_dictionary.search("leetcoded".to_string()), false);
}
