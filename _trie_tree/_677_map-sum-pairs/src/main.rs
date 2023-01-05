/*
* @lc app=leetcode id=677 lang=rust
*
* [677] Map Sum Pairs
*
* https://leetcode.com/problems/map-sum-pairs/description/
*
* algorithms
* Medium (56.99%)
* Likes:    1417
* Dislikes: 138
* Total Accepted:    99.3K
* Total Submissions: 174.5K
* Testcase Example:  '["MapSum","insert","sum","insert","sum"]\n' +
 '[[],["apple",3],["ap"],["app",2],["ap"]]'
*
* Design a map that allows you to do the following:
*
*
* Maps a string key to a given value.
* Returns the sum of the values that have a key with a prefix equal to a given
* string.
*
*
* Implement the MapSum class:
*
*
* MapSum() Initializes the MapSum object.
* void insert(String key, int val) Inserts the key-val pair into the map. If
* the key already existed, the original key-value pair will be overridden to
* the new one.
* int sum(string prefix) Returns the sum of all the pairs' value whose key
* starts with the prefix.
*
*
*
* Example 1:
*
*
* Input
* ["MapSum", "insert", "sum", "insert", "sum"]
* [[], ["apple", 3], ["ap"], ["app", 2], ["ap"]]
* Output
* [null, null, 3, null, 5]
*
* Explanation
* MapSum mapSum = new MapSum();
* mapSum.insert("apple", 3);
* mapSum.sum("ap");           // return 3 (apple = 3)
* mapSum.insert("app", 2);
* mapSum.sum("ap");           // return 5 (apple + app = 3 + 2 = 5)
*
*
*
* Constraints:
*
*
* 1 <= key.length, prefix.length <= 50
* key and prefix consist of only lowercase English letters.
* 1 <= val <= 1000
* At most 50 calls will be made to insert and sum.
*
*
*/

// @lc code=start

#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    value: i32,
}

impl Trie {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, word: String, value: i32) {
        let mut s = self;

        for u in word.as_bytes() {
            s = s.children[*u as usize - 'a' as usize].get_or_insert_with(|| Box::new(Trie::new()));
        }

        s.value = value;
    }
    fn prefix_sum(&self, word: String) -> i32 {
        let mut node = self;
        for u in word.as_bytes() {
            let index = (*u - b'a') as usize;

            if node.children[index].is_none() {
                return 0;
            }
            node = node.children[index].as_ref().unwrap();
        }

        node.get_sum()
    }

    fn get_sum(&self) -> i32 {
        let mut sum = self.value;
       
        for i in 0..26 {
            if let Some(ref child) = self.children[i] {
                sum += child.get_sum();
            }
        }

        sum
    }
}

struct MapSum {
    trie: Trie,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MapSum {
    fn new() -> Self {
        Self { trie: Trie::new() }
    }

    fn insert(&mut self, key: String, val: i32) {
        self.trie.insert(key, val);
    }

    fn sum(&self, prefix: String) -> i32 {
        self.trie.prefix_sum(prefix)
    }
}

/**
 * Your MapSum object will be instantiated and called as such:
 * let obj = MapSum::new();
 * obj.insert(key, val);
 * let ret_2: i32 = obj.sum(prefix);
 */
// @lc code=end

fn main() {
    let mut map_sum = MapSum::new();

    map_sum.insert("apple".to_string(), 3);
    assert_eq!(map_sum.sum("ap".to_string()), 3);
    map_sum.insert("app".to_string(), 2);
    assert_eq!(map_sum.sum("ap".to_string()), 5);

}
