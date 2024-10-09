use std::cmp::Ordering;
use std::ops::Deref;

pub struct BinarySearchTree<T>
where
    T: Ord,
{
    value: Option<T>,
    left: Option<Box<BinarySearchTree<T>>>,
    right: Option<Box<BinarySearchTree<T>>>,
}

impl<T> Default for BinarySearchTree<T>
where
    T: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    pub fn new() -> BinarySearchTree<T> {
        //create rooot node
        BinarySearchTree {
            value: None,
            left: None,
            right: None,
        }
    }

    //Return a new iterator which iterator over the true in order from least to greatest
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        BinarySearchTreeIter::new(self)
    }

    pub fn insert(&mut self, value: T) {
        if self.value.is_none() {
            self.value = Some(value)
        } else {
            match &self.value {
                None => {}
                Some(key) => {
                    let target_node = if value < *key {
                        &mut self.left
                    } else {
                        &mut self.right
                    };
                    match target_node {
                        Some(ref mut node) => node.insert(value),
                        None => {
                            let mut node = BinarySearchTree::new();
                            node.insert(value);
                            *target_node = Some(Box::new(node));
                        }
                    }
                }
            }
        }
    }

    pub fn search(&self, value: &T) -> bool {
        match &self.value {
            Some(key) => {
                match key.cmp(value) {
                    Ordering::Equal => {
                        //key === value
                        true
                    }
                    Ordering::Greater => match &self.left {
                        Some(node) => node.search(value),
                        None => false,
                    },
                    Ordering::Less => {
                        // key < value
                        match &self.right {
                            Some(node) => node.search(value),
                            None => false,
                        }
                    }
                }
            }
            None => false,
        }
    }

    // return smallest value in a tree
    pub fn minimum(&self) -> Option<&T> {
        match &self.left {
            Some(node) => node.minimum(),
            None => self.value.as_ref(),
        }
    }

    pub fn maximum(&self) -> Option<&T> {
        match &self.right {
            Some(node) => node.maximum(),
            None => self.value.as_ref(),
        }
    }

    //floor te largest value in the tree  smaller than the value parameter
    pub fn floor(&self, value: &T) -> Option<&T> {
        match &self.value {
            Some(key) => {
                match key.cmp(value) {
                    Ordering::Greater => {
                        //key > value
                        match &self.left {
                            Some(node) => node.floor(value),
                            None => None,
                        }
                    }
                    Ordering::Less => {
                        //key < value
                        match &self.right {
                            Some(node) => {
                                let val = node.floor(value);
                                match val {
                                    Some(_) => val,
                                    None => Some(key),
                                }
                            }
                            None => Some(key),
                        }
                    }
                    Ordering::Equal => Some(key),
                }
            }
            None => None,
        }
    }
    pub fn ceil(&self, value: &T) -> Option<&T> {
        match &self.value {
            Some(key) => {
                match key.cmp(value) {
                    Ordering::Greater => {
                        // key > value
                        match &self.left {
                            Some(node) => {
                                let val = node.ceil(value);
                                match val {
                                    Some(_) => val,
                                    None => Some(key),
                                }
                            }
                            None => Some(key),
                        }
                    }
                    Ordering::Less => {
                        // key < value
                        match &self.right {
                            Some(node) => node.ceil(value),
                            None => None,
                        }
                    }
                    Ordering::Equal => Some(key),
                }
            }
            None => None,
        }
    }
}

struct BinarySearchTreeIter<'a, T>
where
    T: Ord,
{
    stack: Vec<&'a BinarySearchTree<T>>,
}

impl<'a, T> BinarySearchTreeIter<'a, T>
where
    T: Ord,
{
    pub fn new(tree: &BinarySearchTree<T>) -> BinarySearchTreeIter<T> {
        let mut iter = BinarySearchTreeIter { stack: vec![tree] };
        iter.stack_push_left();
        iter
    }

    fn stack_push_left(&mut self) {
        while let Some(child) = &self.stack.last().unwrap().left {
            self.stack.push(child);
        }
    }
}

impl<'a, T> Iterator for BinarySearchTreeIter<'a, T>
where
    T: Ord,
{
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        if self.stack.is_empty() {
            None
        } else {
            let node = self.stack.pop().unwrap();
            if node.right.is_some() {
                self.stack.push(node.right.as_ref().unwrap().deref());
                self.stack_push_left();
            }
            node.value.as_ref()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    fn create_tree() -> BinarySearchTree<i32> {
        let mut tree = BinarySearchTree::new();
        tree.insert(5);
        tree.insert(43);
        tree.insert(0);
        tree.insert(7);
        tree.insert(27);
        tree.insert(34);
        tree.insert(15);
        tree
    }

    #[test]
    fn test_iterator() {
        //iterate the tree with values ffrom least to greatest
        let tree = create_tree();
        let mut iter = tree.iter();
        assert_eq!(iter.next().unwrap(), &0);
        assert_eq!(iter.next().unwrap(), &5);
        assert_eq!(iter.next().unwrap(), &7);
        assert_eq!(iter.next().unwrap(), &15);
        assert_eq!(iter.next().unwrap(), &27);
        assert_eq!(iter.next().unwrap(), &34);
        assert_eq!(iter.next().unwrap(), &43);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn text_search() {
        let tree = create_tree();
        assert!(tree.search(&5));
        assert!(tree.search(&7));
        assert!(tree.search(&15));
        assert!(!tree.search(&2));
    }

    #[test]
    fn test_max_and_min() {
        let tree = create_tree();
        assert_eq!(*tree.maximum().unwrap(), 43);
        assert_eq!(*tree.minimum().unwrap(), 0);
    }

    #[test]
    fn test_floor_and_ceil() {
        let tree = create_tree();
        assert_eq!(*tree.floor(&42).unwrap(), 34);
        assert_eq!(*tree.floor(&30).unwrap(), 27);

        //test cell method
        assert_eq!(*tree.ceil(&6).unwrap(), 7);
    }
}
