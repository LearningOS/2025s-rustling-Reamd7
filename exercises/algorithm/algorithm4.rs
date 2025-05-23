/*
	二叉搜索树
	此问题要求您实现二叉树的基本接口
*/

use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        if self.root.is_none() {
            self.root = Some(Box::new(TreeNode::new(value)));
        } else {
            self.root.as_mut().unwrap().insert(value);
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        let mut current = &self.root;
        while let Some(node) = current {
            if node.value == value {
                return true;
            } else if value < node.value {
                current = &node.left;
            } else {
                current = &node.right;
            }
        }
        false
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        if value < self.value {
            if self.left.is_none() {
                self.left = Some(Box::new(TreeNode::new(value)));
            } else {
                self.left.as_mut().unwrap().insert(value);
            }
        } else if value > self.value {
            if self.right.is_none() {
                self.right = Some(Box::new(TreeNode::new(value)));
            } else {
                self.right.as_mut().unwrap().insert(value);
            }
        } else {
            // 如果值相等，则不插入
            return;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        // 
        assert_eq!(bst.search(1), false);

        // 
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        // 
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        // 
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        // 
        bst.insert(1);
        bst.insert(1);

        // 
        assert_eq!(bst.search(1), true);

        // 
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}
