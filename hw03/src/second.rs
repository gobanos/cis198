use std::cmp;

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
}