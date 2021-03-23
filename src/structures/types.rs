pub trait Queue<T> {
    fn peek(&self) -> Option<&T>;
    fn enqueue(&mut self, value: T);
    fn dequeue(&mut self) -> Option<T>;
}

pub trait Stack<T> {
    fn peek(&self) -> Option<&T>;
    fn push(&mut self, value: T);
    fn pop(&mut self) -> Option<T>;
}

#[derive(Debug, Eq, PartialEq)]
pub enum SeqError {
    OutOfBound
}

pub trait ReadonlySeq<T> {
    fn get(&self, index: usize) -> Option<&T>;

    fn index_of(&self, value: &T) -> Option<usize> where T: Eq;

    fn size(&self) -> usize;
    fn is_empty(&self) -> bool;
}

pub trait Seq<T>: ReadonlySeq<T> {
    fn insert(&mut self, index: usize, value: T) -> Result<(), SeqError>;

    fn remove_at(&mut self, index: usize) -> Option<T>;

    fn get_mut(&mut self, index: usize) -> Option<&mut T>;
}

pub trait ReadonlyMap<K, V> {
    fn get(&self, key: &K) -> Option<&V>;
    fn size(&self) -> usize;
}

pub trait Map<K, V>: ReadonlyMap<K, V> {
    fn get_mut(&mut self, key: &K) -> Option<&mut V>;
    fn put(&mut self, key: K, value: V);
    fn remove(&mut self, key: &K) -> Option<V>;
}
