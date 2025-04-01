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
    map: HashMap<K, Box<Node<K, V>>>,
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

    pub fn get(&mut self, key: K) -> Option<V> {
        if let Some(node) = self.map.get(&key) {
            let value = node.value;
            self.remove(key);
            self.add(key, value);
            Some(value)
        } else {
            None
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        if self.map.len() >= self.capacity && !self.map.contains_key(&key) {
            // 移除最旧的节点（头节点）
            if let Some(head) = self.head.take() {
                self.map.remove(&head.key);
                self.head = head.next;
                if let Some(ref mut new_head) = self.head {
                    new_head.prev = None;
                } else {
                    self.tail = None;
                }
            }
        }
        
        if self.map.contains_key(&key) {
            self.remove(key);
        }
        self.add(key, value);
    }

    fn add(&mut self, key: K, value: V) {
        let mut new_node = Box::new(Node {
            key,
            value,
            prev: None,
            next: None,
        });

        if let Some(ref mut tail) = self.tail {
            new_node.prev = Some(Box::new(Node {
                key: tail.key,
                value: tail.value,
                prev: tail.prev.take(),
                next: None,
            }));
            tail.next = Some(new_node.clone());
        } else {
            self.head = Some(new_node.clone());
        }
        self.tail = Some(new_node.clone());
        self.map.insert(key, new_node);
    }

    fn remove(&mut self, key: K) {
        if let Some(node) = self.map.remove(&key) {
            match (node.prev, node.next) {
                (Some(mut prev), Some(mut next)) => {
                    // 中间节点
                    prev.next = Some(next.clone());
                    next.prev = Some(prev);
                }
                (None, Some(mut next)) => {
                    // 头节点
                    next.prev = None;
                    self.head = Some(next);
                }
                (Some(mut prev), None) => {
                    // 尾节点
                    prev.next = None;
                    self.tail = Some(prev);
                }
                (None, None) => {
                    // 唯一节点
                    self.head = None;
                    self.tail = None;
                }
            }
        }
    }
}