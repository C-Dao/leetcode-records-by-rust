/*
* @lc app=leetcode id=146 lang=rust
*
* [146] LRU Cache
*
* https://leetcode.com/problems/lru-cache/description/
*
* algorithms
* Medium (40.25%)
* Likes:    16020
* Dislikes: 694
* Total Accepted:    1.2M
* Total Submissions: 3M
* Testcase Example:  '["LRUCache","put","put","get","put","get","put","get","get","get"]\n' +
 '[[2],[1,1],[2,2],[1],[3,3],[2],[4,4],[1],[3],[4]]'
*
* Design a data structure that follows the constraints of a Least Recently
* Used (LRU) cache.
*
* Implement the LRUCache class:
*
*
* LRUCache(int capacity) Initialize the LRU cache with positive size
* capacity.
* int get(int key) Return the value of the key if the key exists, otherwise
* return -1.
* void put(int key, int value) Update the value of the key if the key exists.
* Otherwise, add the key-value pair to the cache. If the number of keys
* exceeds the capacity from this operation, evict the least recently used
* key.
*
*
* The functions get and put must each run in O(1) average time complexity.
*
*
* Example 1:
*
*
* Input
* ["LRUCache", "put", "put", "get", "put", "get", "put", "get", "get", "get"]
* [[2], [1, 1], [2, 2], [1], [3, 3], [2], [4, 4], [1], [3], [4]]
* Output
* [null, null, null, 1, null, -1, null, -1, 3, 4]
*
* Explanation
* LRUCache lRUCache = new LRUCache(2);
* lRUCache.put(1, 1); // cache is {1=1}
* lRUCache.put(2, 2); // cache is {1=1, 2=2}
* lRUCache.get(1);    // return 1
* lRUCache.put(3, 3); // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
* lRUCache.get(2);    // returns -1 (not found)
* lRUCache.put(4, 4); // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
* lRUCache.get(1);    // return -1 (not found)
* lRUCache.get(3);    // return 3
* lRUCache.get(4);    // return 4
*
*
*
* Constraints:
*
*
* 1 <= capacity <= 3000
* 0 <= key <= 10^4
* 0 <= value <= 10^5
* At most 2 * 10^5 calls will be made to get and put.
*
*
*/

// @lc code=start
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};
#[derive(Debug)]
struct ListNode {
    key: i32,
    value: i32,
    next: Option<Rc<RefCell<ListNode>>>,
    prev: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    fn new(key: i32, value: i32) -> Self {
        Self {
            key: key,
            value: value,
            next: None,
            prev: None,
        }
    }
}

struct LRUCache {
    head: Option<Rc<RefCell<ListNode>>>,
    tail: Option<Rc<RefCell<ListNode>>>,
    map: HashMap<i32, Rc<RefCell<ListNode>>>,
    capacity: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        let mut head = Some(Rc::new(RefCell::new(ListNode::new(-1, -1))));
        let mut tail = Some(Rc::new(RefCell::new(ListNode::new(-1, -1))));
        tail.as_mut().unwrap().borrow_mut().prev = head.clone();
        head.as_mut().unwrap().borrow_mut().next = tail.clone();
        Self {
            map: HashMap::new(),
            head,
            tail,
            capacity,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if self.map.contains_key(&key) {
            let node = self.map.get(&key).unwrap().clone();
            self.move_to_tail(node.clone());
            let value = node.borrow().value;
            value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.map.contains_key(&key) {
            let node = self.map.get(&key).unwrap().clone();
            node.borrow_mut().value = value;
            self.move_to_tail(node.clone());
        } else {
            if self.map.len() == self.capacity as usize {
                let will_deleted_node = self
                    .head
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .next
                    .as_ref()
                    .unwrap()
                    .clone();
                self.delete_node(will_deleted_node.clone());
                self.map.remove(&will_deleted_node.borrow().key);
            }

            let node = Rc::new(RefCell::new(ListNode::new(key, value)));
            self.insert_to_tail(node.clone());
            self.map.insert(key, node);
        }
    }

    fn move_to_tail(&mut self, node: Rc<RefCell<ListNode>>) {
        self.delete_node(node.clone());
        self.insert_to_tail(node.clone());
    }

    fn delete_node(&mut self, node: Rc<RefCell<ListNode>>) {
        let mut next = node.borrow_mut().next.take();
        let mut prev = node.borrow_mut().prev.take();
        prev.as_mut().unwrap().borrow_mut().next = next.clone();
        next.as_mut().unwrap().borrow_mut().prev = prev.clone();
    }

    fn insert_to_tail(&mut self, node: Rc<RefCell<ListNode>>) {
        self.tail
            .as_mut()
            .unwrap()
            .borrow_mut()
            .prev
            .as_mut()
            .unwrap()
            .borrow_mut()
            .next = Some(node.clone());
        node.borrow_mut().prev = self.tail.as_ref().unwrap().borrow().prev.clone();
        node.borrow_mut().next = self.tail.clone();
        self.tail.as_mut().unwrap().borrow_mut().prev = Some(node);
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
// @lc code=end

fn main() {
    let mut lru_cache = LRUCache::new(2);
    lru_cache.put(1, 1);
    lru_cache.put(2, 2);
    assert_eq!(lru_cache.get(1), 1);
    lru_cache.put(3, 3);
    assert_eq!(lru_cache.get(2), -1);
    lru_cache.put(4, 4);
    assert_eq!(lru_cache.get(1), -1);
    assert_eq!(lru_cache.get(3), 3);
    assert_eq!(lru_cache.get(4), 4);
}
