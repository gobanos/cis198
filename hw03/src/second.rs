use std::cmp;

pub struct IntoIter<T>(BST<T>);
pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}
pub struct IterMut<'a, T: 'a> {
    next: Option<&'a mut Node<T>>,
}

#[derive(Debug)]
pub struct BST<T> {
    root: Link<T>,
}

impl<T: cmp::PartialOrd> BST<T> {
    pub fn new() -> Self {
        BST {
            root: None,
        }
    }

    pub fn insert(&mut self, val: T) -> bool {
        self.root.insert(val)
    }

    pub fn search(&self, val: T) -> bool {
        self.root.search(val)
    }
}

impl<T> IntoIterator for BST<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

impl<'a, T> IntoIterator for &'a BST<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        Iter { next: self.root.as_ref().map(|node| &**node) }
    }
}

impl<'a, T> IntoIterator for &'a mut BST<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        IterMut { next: self.root.as_mut().map(|node| &mut **node) }
    }
}

trait InsertSearch<T: cmp::PartialOrd> {
    fn insert(&mut self, val: T) -> bool;
    fn search(&self, val: T) -> bool;
}

type Link<T> = Option<Box<Node<T>>>;

impl<T: cmp::PartialOrd> InsertSearch<T> for Link<T> {
    fn insert(&mut self, val: T) -> bool {
        match self {
            &mut None => {
                *self = Some(Box::new(Node { elem: val, left: None, right: None }));
                true
            },
            &mut Some(ref mut n) => {
                if n.elem > val {
                    n.left.insert(val)
                } else if n.elem < val {
                    n.right.insert(val)
                } else {
                    false
                }
            }
        }
    }

    fn search(&self, val: T) -> bool {
        match self {
            &None => {
                false
            },
            &Some(ref n) => {
                if n.elem > val {
                    n.left.search(val)
                } else if n.elem < val {
                    n.right.search(val)
                } else {
                    true
                }
            }
        }
    }
}

#[derive(Debug)]
struct Node<T> {
    elem: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.root.take().map(|box Node {elem, right, ..}| {
            self.0.root = right.map(|node| node);
            elem
        })
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.right.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.right.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::BST;

    #[test]
    fn test_push_pop() {
        let mut bst = BST::new();

        assert!(!bst.search(5));

        assert!(bst.insert(5));
        assert!(bst.search(5));
    }

    #[test]
    fn test_unordered_inserts() {
        let mut bst = BST::new();
        assert!(bst.insert(5));
        assert!(bst.insert(1));
        assert!(bst.insert(2));
        assert!(bst.insert(4));
        assert!(bst.insert(3));

        assert!(bst.search(1));
        assert!(bst.search(2));
        assert!(bst.search(3));
        assert!(bst.search(4));
        assert!(bst.search(5));
    }

    #[test]
    fn test_multiple_inserts() {
        let mut bst = BST::new();
        assert!(bst.insert(1));
        assert!(!bst.insert(1));
    }

    #[test]
    fn into_iter() {
        let mut bst = BST::new();
        assert!(bst.insert(1));
        assert!(bst.insert(2));
        assert!(bst.insert(3));

        let mut iter = bst.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut bst = BST::new();
        assert!(bst.insert(1));
        assert!(bst.insert(2));
        assert!(bst.insert(3));

        let mut iter = (&bst).into_iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut bst = BST::new();
        assert!(bst.insert(1));
        assert!(bst.insert(2));
        assert!(bst.insert(3));

        let mut iter = (&mut bst).into_iter();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
    }
}