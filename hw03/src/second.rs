#[derive(Debug)]
pub struct BST {
    root: Link,
}

impl BST {
    pub fn new() -> BST {
        BST {
            root: Link::Empty,
        }
    }

    pub fn insert(&mut self, val: i32) -> bool {
        self.root.insert(val)
    }

    pub fn search(&self, val: i32) -> bool {
        self.root.search(val)
    }
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

impl Link {
    fn insert(&mut self, val: i32) -> bool {
        match self {
            &mut Link::Empty => {
                *self = Link::More(Box::new(Node { elem: val, left: Link::Empty, right: Link::Empty }));
                true
            },
            &mut Link::More(ref mut n) => {
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

    fn search(&self, val: i32) -> bool {
        match self {
            &Link::Empty => {
                false
            },
            &Link::More(ref n) => {
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
struct Node {
    elem: i32,
    left: Link,
    right: Link,
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