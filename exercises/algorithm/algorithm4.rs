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
        //TODO
        //判断是否有根，如果有则调用根节点的insert方法，没有则创建根节点
        if let Some(root) = &mut self.root {
            //判断根节点是否是插入的值，如果不是再插入
            if root.value.cmp(&value) == Ordering::Equal{
                return;
            }else{
                root.insert(value);
            }
        }else {
            self.root = Some(Box::new(TreeNode::new(value)));
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        let mut cut = &self.root;

        while let Some(node) = cut {
            match node.value.cmp(&value) {
                //查找的值较大，在右子树中搜索
                Ordering::Less => cut = &node.right,
                //查找的值较小，在左子树中搜索
                Ordering::Greater => cut = &node.left,
                //找到目标值
                Ordering::Equal => return true,
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
        //比较值大小，插入的值小于当前节点时向左子树搜索，大于等于时向右子树搜索
        match self.value.cmp(&value) {
            //当前节点值小于插入节点的值
            Ordering::Less => {
                //判断是否存在右节点
                if let Some(r_node) = &mut self.right {
                    //递归插入
                    r_node.insert(value);
                } else {
                    //右节点不存在则插入
                    self.right = Some(Box::new(TreeNode::new(value)));
                }
            }
            Ordering::Equal | Ordering::Greater => {
                //判断是否存在左节点
                if let Some(l_node) = &mut self.left {
                    //递归插入
                    l_node.insert(value);
                } else {
                    //右节点不存在则插入
                    self.left = Some(Box::new(TreeNode::new(value)));
                }
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
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}
