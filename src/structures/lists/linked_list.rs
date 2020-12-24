use crate::structures::types::*;

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new_link(value: T, next: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
        Some(Box::new(Node::new(value, next)))
    }

    fn new(value: T, next: Option<Box<Node<T>>>) -> Node<T> {
        Node {
            value,
            next,
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
        }
    }

    pub fn from_value(value: T) -> LinkedList<T> {
        LinkedList {
            head: Node::new_link(value, None)
        }
    }
}


impl<T> Queue<T> for LinkedList<T> {
    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node_ref| &node_ref.value)
    }

    fn enqueue(&mut self, value: T) {
        let mut current = &mut self.head;

        while let Some(node_ref) = current {
            let node = &mut **node_ref;
            current = &mut node.next;
        }

        *current = Node::new_link(value, None);
    }

    fn dequeue(&mut self) -> Option<T> {
        let node_ref = self.head.take();
        match node_ref {
            Some(node_box) => {
                let mut node = *node_box;
                self.head = node.next.take();
                Some(node.value)
            }
            None => None,
        }
    }
}

impl<T> Stack<T> for LinkedList<T> {
    fn peek(&self) -> Option<&T> {
        Queue::peek(self)
    }

    fn push(&mut self, value: T) {
        self.head = Node::new_link(value, self.head.take());
    }

    fn pop(&mut self) -> Option<T> {
        self.dequeue()
    }
}

impl<T> ReadonlySeq<T>for LinkedList<T> {
    fn get(&self, index: usize) -> Option<&T> {
        let mut current_index: usize = 0;
        let mut current = &self.head;
        while let Some(node_ref) = current {
            let node = &**node_ref;
            if index == current_index {
                return Some(&node.value);
            }

            current = &node.next;
            current_index += 1;
        }
        None
    }

    fn index_of(&self, value: &T) -> Option<usize> where T: Eq {
        let mut index: usize = 0;
        let mut current = &self.head;
        while let Some(node_ref) = current {
            let node = &**node_ref;
            if node.value.eq(value) {
                return Some(index);
            }
            current = &node.next;
            index += 1;
        }
        None
    }

    fn size(&self) -> usize {
        let mut size: usize = 0;
        let mut current = &self.head;
        while let Some(node_ref) = current {
            size += 1;
            current = &(**node_ref).next;
        }
        size
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

}

impl<T> Seq<T> for LinkedList<T> {
    fn insert(&mut self, index: usize, value: T) -> Result<(), SeqError> {
        let mut current_index = 0;
        let mut current = &mut self.head;

        loop {
            current = match current.take() {
                Some(node_ref) => {
                    if current_index == index {
                        *current = Node::new_link(value, Some(node_ref));
                        return Ok(());
                    }

                    current_index += 1;
                    current.replace(node_ref);
                    match current.as_mut() {
                        Some(node_ref) => &mut (**node_ref).next,
                        None => break,
                    }
                }
                None => break
            }
        }
        return Err(SeqError::OutOfBound);
    }

    fn remove_at(&mut self, index: usize) -> Option<T> {
        let mut current_index = 0;
        let mut current = &mut self.head;

        loop {
            current = match current.take() {
                Some(mut node_ref) => {
                    let node = &mut *node_ref;

                    if current_index == index {
                        *current = node.next.take();
                        return Some(node_ref.value);
                    }

                    current_index += 1;
                    current.replace(node_ref);
                    match current.as_mut() {
                        Some(node_ref) => &mut (**node_ref).next,
                        None => break,
                    }
                }
                None => break
            }
        }
        return None;
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        let mut current_index: usize = 0;
        let mut current = &mut self.head;
        while let Some(node_ref) = current {
            let node = &mut **node_ref;
            if index == current_index {
                return Some(&mut node.value);
            }

            current = &mut node.next;
            current_index += 1;
        }
        None
    }

}


impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new() {
        let list = LinkedList::<i32>::new();
        assert_eq!(0, list.size());
        assert_eq!(None, list.get(0));
    }

    #[test]
    fn create_from() {
        let list = LinkedList::from_value(1000);
        assert_eq!(1, list.size());
        assert_eq!(Some(&1000), list.get(0));
    }



    #[test]
    fn index_of() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(Some(1usize), list.index_of(&2));
    }


    #[test]
    fn test() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        assert_eq!(Ok(()), list.insert(0, 10));
        println!("{:?}", list);
    }
}
