use std::borrow::BorrowMut;
use std::cell::UnsafeCell;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::mem::{MaybeUninit, replace};
use std::rc::{Rc, Weak};

use crate::structures::types::{Map, ReadonlyMap};

use self::RemovingStatus::{FoundLeft, FoundRight, FoundRoot, Searching, SearchingLeft, SearchingRight};

#[derive(Debug)]
struct Node<K, V> {
    key: K,
    value: MaybeUninit<V>,
    parent: Option<Weak<UnsafeCell<Self>>>,
    left: Option<Rc<UnsafeCell<Self>>>,
    right: Option<Rc<UnsafeCell<Self>>>,
}

#[derive(Debug)]
pub struct BinarySearchTree<K, V> {
    root: Option<Rc<UnsafeCell<Node<K, V>>>>,
    size: usize,
}

impl<K, V> Node<K, V> {
    fn new(key: K, value: V, parent: Option<Weak<UnsafeCell<Self>>>) -> Self {
        Node {
            key,
            value: MaybeUninit::new(value),
            parent,
            left: None,
            right: None,
        }
    }

    fn new_link(key: K, value: V, parent: Option<Weak<UnsafeCell<Self>>>) -> Option<Rc<UnsafeCell<Self>>> {
        Some(Rc::new(UnsafeCell::new(Self::new(key, value, parent))))
    }

    fn select_left_mut(&mut self) -> &mut Option<Rc<UnsafeCell<Self>>> {
        &mut self.left
    }

    fn select_right_mut(&mut self) -> &mut Option<Rc<UnsafeCell<Self>>> {
        &mut self.right
    }
}

impl<K, V> BinarySearchTree<K, V> {
    fn new() -> Self {
        BinarySearchTree {
            root: None,
            size: 0,
        }
    }
}

fn get_ref<K, V>(rc: &Rc<UnsafeCell<Node<K, V>>>) -> &Node<K, V> {
    unsafe { &*(**rc).get() }
}

fn get_mut<K, V>(rc: &mut Rc<UnsafeCell<Node<K, V>>>) -> &mut Node<K, V> {
    unsafe { &mut *(**rc).get() }
}

fn rebalance<K, V>(
    node: &mut Node<K, V>,
    kind: NodeKind,
) -> Option<Rc<UnsafeCell<Node<K, V>>>> {
    *&mut node.parent = None;

    let node_selector = match kind {
        NodeKind::Root |
        NodeKind::Left => Node::select_left_mut,
        NodeKind::Right => Node::select_right_mut,
    };

    let mut right = node.right.take();
    let mut left = node.left.take();

    let mut curr = match kind {
        NodeKind::Root => {
            match (right.is_some(), left.is_some()) {
                (true, false) => return right,
                (false, true) => return left,
                (false, false) => return None,
                (true, true) => &mut right,
            }
        }
        NodeKind::Left => &mut right,
        NodeKind::Right => &mut left,
    };

    loop {
        let found = match curr {
            Some(rc) => {
                curr = node_selector(get_mut(rc));
                curr.as_mut()
                    .map(|rc| node_selector(get_mut(rc)).is_none())
                    .unwrap_or(true)
            }
            None => return None,
        };

        if found { break; }
    }

    return curr.take()
        .map(|mut next_parent| {
            let link = node_selector(get_mut(&mut next_parent));
            *link = match kind {
                NodeKind::Root |
                NodeKind::Left => left,
                NodeKind::Right => right,
            };
            next_parent
        });
}

fn remove_node<K, V>(
    link: &mut Option<Rc<UnsafeCell<Node<K, V>>>>,
    kind: NodeKind,
) -> Option<V> {
    let owned_link = link.take();
    match owned_link {
        Some(mut rc) => {
            let mut node = get_mut(&mut rc);
            let next = rebalance(node, kind);
            *link = next;
            if let Some(mut rc) = link.as_mut() {
                let child = get_mut(rc);
                *&mut child.parent = node.parent.take();
            }

            return Some(
                unsafe {
                    replace(
                        &mut node.value,
                        MaybeUninit::uninit(),
                    )
                        .assume_init()
                }
            );
        }
        None => unreachable!()
    };
}

fn resolve_status<K, V>(
    link: &Option<Rc<UnsafeCell<Node<K, V>>>>,
    key: &K,
    found: RemovingStatus,
    searching: RemovingStatus,
) -> Option<RemovingStatus>
    where K: Eq
{
    link.as_ref()
        .map(|rc| if get_ref(rc).key.eq(&key) { found } else { searching })
}

impl<K, V> BinarySearchTree<K, V> {
    fn increment_size(&mut self) {
        self.size += 1;
    }

    fn decrement_size(&mut self) {
        self.size -= 1;
    }
}

impl<K, V> ReadonlyMap<K, V> for BinarySearchTree<K, V> where K: Ord, V: Debug {
    fn get(&self, key: &K) -> Option<&V> {
        let mut curr = &self.root;
        while let Some(node_rc) = curr {
            let node = get_ref(node_rc);

            match node.key.cmp(key) {
                Ordering::Equal => return Some(unsafe { &*node.value.as_ptr() }),
                Ordering::Less => curr = &node.right,
                Ordering::Greater => curr = &node.left,
            }
        }

        None
    }

    fn size(&self) -> usize {
        self.size
    }
}

impl<K, V> Map<K, V> for BinarySearchTree<K, V> where K: Ord, V: Debug {
    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let mut curr = &mut self.root;
        while let Some(node_rc) = curr {
            let node = get_mut(node_rc);

            match node.key.cmp(key) {
                Ordering::Equal => return Some(unsafe { &mut *node.value.as_mut_ptr() }),
                Ordering::Less => curr = &mut node.right,
                Ordering::Greater => curr = &mut node.left,
            }
        }

        None
    }

    fn put(&mut self, key: K, value: V) {
        let mut curr = &mut self.root;

        loop {
            match curr {
                Some(node_rc) => {
                    let weak = Rc::downgrade(node_rc);
                    let node = get_mut(node_rc);

                    match (node.key.cmp(&key), node.left.is_some(), node.right.is_some()) {
                        (Ordering::Equal, _, _) => {
                            *node.value.borrow_mut() = MaybeUninit::new(value);

                            return;
                        }
                        (Ordering::Greater, true, _) => curr = &mut node.left,
                        (Ordering::Greater, false, _) => {
                            *&mut node.left = Node::new_link(key, value, Some(weak));
                            self.increment_size();
                            return;
                        }
                        (Ordering::Less, _, true) => curr = &mut node.right,
                        (Ordering::Less, _, false) => {
                            *&mut node.right = Node::new_link(key, value, Some(weak));
                            self.increment_size();
                            return;
                        }
                    }
                }
                None => {
                    *curr = Some(Rc::new(UnsafeCell::new(Node::new(key, value, None))));
                    self.increment_size();
                    return;
                }
            }
        }
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        let mut status = resolve_status(&self.root, key, FoundRoot, Searching)?;

        if let FoundRoot = status {
            let res = remove_node(&mut self.root, NodeKind::Root);
            if res.is_some() { self.decrement_size(); }
            return res;
        }

        let mut parent = &mut self.root;


        loop {
            match parent {
                Some(rc) => {
                    let parent_node = get_mut(rc);

                    status = match key.cmp(&parent_node.key) {
                        Ordering::Equal => unreachable!(),
                        Ordering::Greater => resolve_status(&parent_node.right, key, FoundRight, SearchingRight),
                        Ordering::Less => resolve_status(&parent_node.left, key, FoundLeft, SearchingLeft),
                    }?;


                    parent = match status {
                        FoundRoot | Searching => unreachable!(),
                        SearchingLeft => &mut parent_node.left,
                        SearchingRight => &mut parent_node.right,

                        FoundLeft => {
                            let res = remove_node(&mut parent_node.left, NodeKind::Left);
                            if res.is_some() { self.decrement_size(); }
                            return res;
                        }

                        FoundRight => {
                            let res = remove_node(&mut parent_node.right, NodeKind::Right);
                            if res.is_some() { self.decrement_size(); }
                            return res;
                        }
                    }
                }
                None => unreachable!()
            }
        }
    }
}

#[derive(Copy, Clone)]
enum NodeKind {
    Left,
    Right,
    Root,
}

#[derive(Copy, Clone)]
enum RemovingStatus {
    FoundRoot,
    Searching,
    FoundLeft,
    FoundRight,
    SearchingLeft,
    SearchingRight,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn add_number(tree: &mut BinarySearchTree<i32, String>, value: i32) {
        tree.put(value, value.to_string());
    }

    fn create_tree() -> BinarySearchTree<i32, String> {
        let mut tree = BinarySearchTree::new();
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
    fn create() {
        let tree = BinarySearchTree::<i32, String>::new();

        assert_eq!(0, tree.size());
    }

    #[test]
    fn add1() {
        let mut tree = BinarySearchTree::new();
        tree.put(20, "twenty".to_string());
        tree.put(10, "ten".to_string());
        assert_eq!(2, tree.size());

        assert_eq!(Some(&"twenty".to_string()), tree.get(&20));
        assert_eq!(Some(&"ten".to_string()), tree.get(&10));
    }

    #[test]
    fn get_mut1() {
        let mut tree = BinarySearchTree::new();
        tree.put(20, "twenty".to_string());
        tree.put(10, "ten".to_string());
        assert_eq!(2, tree.size());

        assert_eq!(Some(&"ten".to_string()), tree.get(&10));

        *tree.get_mut(&10).unwrap() = "smth".to_string();

        assert_eq!(Some(&"smth".to_string()), tree.get(&10));
    }

    #[test]
    fn remove1() {
        let mut tree = BinarySearchTree::new();
        tree.put(20, "twenty".to_string());
        tree.put(10, "ten".to_string());
        assert_eq!(2, tree.size());

        let val = tree.remove(&10);
        assert_eq!(Some("ten".to_string()), val);
        assert_eq!(None, tree.get(&10));
        assert_eq!(1, tree.size);
    }


    #[test]
    fn remove_with_left() {
        let mut tree = create_tree();
        let mut expected = [17, 28, 29, 44, 54, 65, 76, 78, 80, 82, 88, 97].iter();

        assert_eq!(tree.remove(&32), Some("32".to_string()));
        // tree.traverse(
        //     &mut |key, _| assert_eq!(expected.next(), Some(key)),
        //     TraverseDirection::LNR,
        // );
    }

    #[test]
    fn remove_with_right() {
        let mut tree = create_tree();
        let mut expected = [28, 29, 32, 44, 54, 65, 76, 78, 80, 82, 88, 97].iter();

        assert_eq!(tree.remove(&17), Some("17".to_string()));
        // tree.traverse(
        //     &mut |key, _| assert_eq!(expected.next(), Some(key)),
        //     TraverseDirection::LNR,
        // );
    }


    #[test]
    fn remove_with_both() {
        let mut tree = create_tree();
        let mut expected = [17, 28, 29, 32, 44, 54, 76, 78, 80, 82, 88, 97].iter();
        assert_eq!(Some("65".to_string()), tree.remove(&65));

        // tree.traverse(
        //     &mut |key, _| assert_eq!(expected.next(), Some(key)),
        //     TraverseDirection::LNR,
        // );
    }

    #[test]
    fn get() {
        let tree = create_tree();
        assert_eq!(Some(&54.to_string()), tree.get(&54));
    }

    #[test]
    fn add() {
        let mut tree = BinarySearchTree::new();
        assert_eq!(None, tree.get(&1));
        tree.put(1, 4000);
        assert_eq!(Some(&4000), tree.get(&1));
    }
}
