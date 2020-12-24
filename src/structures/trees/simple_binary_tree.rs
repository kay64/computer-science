use std::cmp::Ordering;
use std::fmt::Debug;

enum Either<L, R> {
    Left(L),
    Right(R),
}

#[derive(Debug)]
struct Node<K, V>
    where
        K: Ord + Debug,
        V: Debug {
    key: K,
    value: V,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
}

#[derive(Debug)]
pub struct BinaryTree<K, V>
    where
        K: Ord + Debug,
        V: Debug,
{
    root: Option<Box<Node<K, V>>>,
}

pub enum TraverseDirection {
    NLR,
    LNR,
    LRN,
}

impl<K, V> Node<K, V>
    where
        K: Ord + Debug,
        V: Debug,
{
    fn new_link(key: K, value: V) -> Option<Box<Node<K, V>>> {
        Some(Box::new(Node {
            key,
            value,
            left: None,
            right: None,
        }))
    }

    fn traverse_nlr<F>(&self, func: &mut F) where F: FnMut(&K, &V) {
        func(&self.key, &self.value);
        if let Some(left_ptr) = self.left.as_ref() {
            left_ptr.traverse_nlr(func);
        }
        if let Some(right_ptr) = self.right.as_ref() {
            right_ptr.traverse_nlr(func);
        }
    }

    fn traverse_lnr<F>(&self, func: &mut F) where F: FnMut(&K, &V) {
        if let Some(left_ptr) = self.left.as_ref() {
            left_ptr.traverse_lnr(func);
        }
        func(&self.key, &self.value);
        if let Some(right_ptr) = self.right.as_ref() {
            right_ptr.traverse_lnr(func);
        }
    }

    fn traverse_lrn<F>(&self, func: &mut F) where F: FnMut(&K, &V) {
        if let Some(left_ptr) = self.left.as_ref() {
            left_ptr.traverse_lrn(func);
        }
        if let Some(right_ptr) = self.right.as_ref() {
            right_ptr.traverse_lrn(func);
        }
        func(&self.key, &self.value);
    }
}

impl<K, V> BinaryTree<K, V>
    where
        K: Ord + Debug,
        V: Debug,
{
    pub fn new() -> BinaryTree<K, V> {
        return BinaryTree { root: None };
    }

    pub fn add(&mut self, key: K, value: V) {
        let mut node_link = &mut self.root;

        while let Some(node_ptr) = node_link {
            let node = &mut **node_ptr;
            node_link = match key.cmp(&node.key) {
                Ordering::Less => &mut node.left,
                _ => &mut node.right,
            }
        }

        *node_link = Node::new_link(key, value);
    }

    pub fn get(&self, key: K) -> Option<&V> {
        let mut node_link = &self.root;

        while let Some(node_ptr) = node_link {
            let node = &**node_ptr;

            node_link = match key.cmp(&node.key) {
                Ordering::Equal => return Some(&node.value),
                Ordering::Less => &node.left,
                Ordering::Greater => &node.right,
            }
        }

        None
    }

    pub fn remove(&mut self, key: K) -> Option<V> {
        let mut node_link = &mut self.root;


        while node_link.is_some() {
            let node_box = node_link.take()?;

            if (*node_box).key == key {
                return BinaryTree::remove_node(node_link, node_box);
            }

            node_link.replace(node_box);


            let node_box = node_link.as_mut()?;
            node_link = match key.cmp(&(**node_box).key) {
                Ordering::Less => &mut (**node_box).left,
                Ordering::Greater => &mut (**node_box).right,
                Ordering::Equal => break,
            }
        }
        None
    }

    fn remove_node(node_ref: &mut Option<Box<Node<K, V>>>,
                   mut node_box: Box<Node<K, V>>) -> Option<V> {
        match (node_box.right.take(), node_box.left.take()) {
            (Some(mut right_box), Some(left_box)) => {
                let mut next = &mut right_box;
                loop {
                    let res = match next.left.take() {
                        Some(node_box) => {
                            if (*node_box).left.is_none() {
                                Either::Left(node_box)
                            } else {
                                next.left.replace(node_box);
                                Either::Right(next.left.as_mut()?)
                            }
                        }
                        None => break,
                    };

                    next = match res {
                        Either::Left(mut value) => {
                            let right = value.right.take();
                            value.left = Some(left_box);
                            value.right = Some(right_box);
                            if right.is_some() {
                                let mut leftest = &mut value.right;
                                while let Some(next) = leftest {
                                    leftest = &mut (**next).left;
                                }
                                *leftest = right;
                            }
                            *node_ref = Some(value);
                            break;
                        }
                        Either::Right(value) => value,
                    }
                }
            }
            (Some(right_box), None) => { *node_ref = Some(right_box); }
            (None, Some(left_box)) => { *node_ref = Some(left_box); }
            (None, None) => {}
        };

        return Some(node_box.value);
    }

    pub fn traverse<F>(&self, func: &mut F, direction: TraverseDirection) where F: FnMut(&K, &V) {
        match self.root.as_ref() {
            Some(node_ref) => {
                let node = &**node_ref;
                match direction {
                    TraverseDirection::NLR => node.traverse_nlr(func),
                    TraverseDirection::LNR => node.traverse_lnr(func),
                    TraverseDirection::LRN => node.traverse_lrn(func),
                }
            }
            None => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn add_number(tree: &mut BinaryTree<i32, String>, value: i32) {
        tree.add(value, value.to_string());
    }

    fn create_tree() -> BinaryTree<i32, String> {
        let mut tree = BinaryTree::new();
        let tree_ref = &mut tree;
        add_number(tree_ref, 44);
        add_number(tree_ref, 17);
        add_number(tree_ref, 88);
        add_number(tree_ref, 32);
        add_number(tree_ref, 65);
        add_number(tree_ref, 97);
        add_number(tree_ref, 28);
        add_number(tree_ref, 54);
        add_number(tree_ref, 82);
        add_number(tree_ref, 29);
        add_number(tree_ref, 76);
        add_number(tree_ref, 80);
        add_number(tree_ref, 78);
        tree
    }

    #[test]
    fn remove_with_left() {
        let mut tree = create_tree();
        let mut expected = [17, 28, 29, 44, 54, 65, 76, 78, 80, 82, 88, 97].iter();

        assert_eq!(tree.remove(32), Some("32".to_string()));
        tree.traverse(
            &mut |key, _| assert_eq!(expected.next(), Some(key)),
            TraverseDirection::LNR,
        );
    }

    #[test]
    fn remove_with_right() {
        let mut tree = create_tree();
        let mut expected = [28, 29, 32, 44, 54, 65, 76, 78, 80, 82, 88, 97].iter();

        assert_eq!(tree.remove(17), Some("17".to_string()));
        tree.traverse(
            &mut |key, _| assert_eq!(expected.next(), Some(key)),
            TraverseDirection::LNR,
        );
    }


    #[test]
    fn remove_with_both() {
        let mut tree = create_tree();
        let mut expected = [17, 28, 29, 32, 44, 54, 76, 78, 80, 82, 88, 97].iter();
        assert_eq!(Some("65".to_string()), tree.remove(65));

        tree.traverse(
            &mut |key, _| assert_eq!(expected.next(), Some(key)),
            TraverseDirection::LNR,
        );
    }

    #[test]
    fn get() {
        let tree = create_tree();
        assert_eq!(Some(&54.to_string()), tree.get(54));
    }

    #[test]
    fn add() {
        let mut tree = BinaryTree::new();
        assert_eq!(None, tree.get(1));
        tree.add(1, 4000);
        assert_eq!(Some(&4000), tree.get(1));
    }

    #[test]
    fn traverse_inorder() {
        let tree = create_tree();
        let mut expected = [17, 28, 29, 32, 44, 54, 65, 76, 78, 80, 82, 88, 97].iter();

        tree.traverse(
            &mut |key, _| assert_eq!(expected.next(), Some(key)),
            TraverseDirection::LNR,
        );
    }

    #[test]
    fn traverse_postorder() {
        let tree = create_tree();
        let mut expected = [29, 28, 32, 17, 54, 78, 80, 76, 82, 65, 97, 88, 44].iter();

        tree.traverse(
            &mut |key, _| assert_eq!(expected.next(), Some(key)),
            TraverseDirection::LRN,
        );
    }

    #[test]
    fn traverse_preorder() {
        let tree = create_tree();
        let mut expected = [44, 17, 32, 28, 29, 88, 65, 54, 82, 76, 80, 78, 97].iter();

        tree.traverse(
            &mut |key, _| assert_eq!(expected.next(), Some(key)),
            TraverseDirection::NLR,
        );
    }
}
