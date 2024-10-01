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
    fn test_iterator(){//iterate the tree with values ffrom least to greatest
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
}
