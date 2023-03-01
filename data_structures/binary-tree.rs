pub struct BinaryTree<T> {
    root: Option<Box<Node<T>>>,
}

pub struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> BinaryTree<T> {
    pub fn new() -> Self {
        BinaryTree { root: None }
    }

    pub fn insert(&mut self, value: T)
    where
        T: PartialOrd + Copy,
    {
        let mut current = &mut self.root;
        while let Some(node) = current {
            if value < node.value {
                current = &mut node.left;
            } else {
                current = &mut node.right;
            }
        }
        *current = Some(Box::new(Node {
            value,
            left: None,
            right: None,
        }));
    }

    pub fn contains(&self, value: T) -> bool
    where
        T: PartialOrd + Copy,
    {
        let mut current = &self.root;
        while let Some(node) = current {
            if value < node.value {
                current = &node.left;
            } else if value > node.value {
                current = &node.right;
            } else {
                return true;
            }
        }
        false
    }
}

fn main() {
    let mut tree = BinaryTree::new();
    tree.insert(5);
    tree.insert(3);
    tree.insert(7);
    tree.insert(1);
    tree.insert(4);
    tree.insert(6);
    tree.insert(8);
    println!("{}", tree.contains(5));
    println!("{}", tree.contains(3));
    println!("{}", tree.contains(7));
    println!("{}", tree.contains(1));
    println!("{}", tree.contains(4));
    println!("{}", tree.contains(6));
    println!("{}", tree.contains(8));
    println!("{}", tree.contains(2));
}
