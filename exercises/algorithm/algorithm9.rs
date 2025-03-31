/*
	linked_list
	这个问题要求你使用链表来实现LRU算法
*/

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Node<K, V> {
    key: K,
    value: V,
    prev: Option<Box<Node<K, V>>>,
    next: Option<Box<Node<K, V>>>,
}

#[derive(Debug)]
pub struct LRUCache<K, V> {
    capacity: usize,
    map: HashMap<K, V>,
    head: Option<Box<Node<K, V>>>,
    tail: Option<Box<Node<K, V>>>,
}

impl<K, V> LRUCache<K, V>
where
    K: Eq + PartialEq + std::hash::Hash + Copy + Clone,
    V: Copy + Clone,
{
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            map: HashMap::new(),
            head: None,
            tail: None,
        }
    }

    pub fn get(&mut self, key: K) -> Option<&V> {
        if let Some(value) = self.map.get(&key) {
            self.remove(key);
            self.add(key, value);
            Some(value)
        } else {
            None
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        //TODO
        if let Some(old_value) = self.map.get(&key) {
            self.remove(key);
            self.add(key, value);
        } else {
            self.add(key, value);
        }
    }

    fn add(&mut self, key: K, value: V) {
        let new_node = Box::new(Node {
            key,
            value,
            prev: None,
            next: None,
        });
        if let Some(tail) = self.tail.take().as_mut() {
            tail.next = Some(new_node);
            new_node.prev = Some(tail);
        } else {
            self.head = Some(new_node);
        }
        self.tail = Some(new_node);
    }

    fn remove(&mut self, key: K) {
        if let Some(node) = self.map.remove(&key) {
            if let Some(prev) = node.prev {
                prev.next = node.next;
            } else {
                self.head = node.next;
            }
            if let Some(next) = node.next {
                next.prev = node.prev;
            } else {
                self.tail = node.prev;
            }   
        }
    }
}