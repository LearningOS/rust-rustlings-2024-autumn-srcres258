/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
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
        match self.root.as_mut() {
            Some(root) => {
                // Ask the root node to insert the value.
                root.insert(value);
            }
            None => {
                // Initialise the root node if not created yet.
                let root = Box::new(TreeNode {
                    value,
                    left: None,
                    right: None
                });
                self.root = Some(root);
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        match self.root.as_ref() {
            None => false,
            Some(root) => {
                let mut cur_node = Some(root);
                while let Some(node) = cur_node {
                    match value.cmp(&node.value) {
                        Ordering::Equal => {
                            return true;
                        }
                        Ordering::Less => {
                            cur_node = node.left.as_ref();
                        }
                        Ordering::Greater => {
                            cur_node = node.right.as_ref();
                        }
                    }
                }
                false
            }
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Less => {
                match self.left.as_mut()  {
                    Some(left) => {
                        // If the left node exists, call its insert method
                        // to ask it to insert the value.
                        left.insert(value);
                    }
                    None => {
                        // If the left node is missing, create one and record it.
                        let child = Box::new(TreeNode {
                            value,
                            left: None,
                            right: None
                        });
                        self.left = Some(child);
                    }
                }
            }
            Ordering::Greater => {
                match self.right.as_mut()  {
                    Some(right) => {
                        // If the right node exists, call its insert method
                        // to ask it to insert the value.
                        right.insert(value);
                    }
                    None => {
                        // If the right node is missing, create one and record it.
                        let child = Box::new(TreeNode {
                            value,
                            left: None,
                            right: None
                        });
                        self.right = Some(child);
                    }
                }
            }
            Ordering::Equal => {
                // For the equal circumstance, do no actions
                // since self is already the given value.
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


