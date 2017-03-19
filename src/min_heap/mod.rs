use std::collections::BinaryHeap;
use std::collections::binary_heap::{PeekMut};
use std::ops::DerefMut;
use std::cmp::{Ordering};
use std::iter::Map;

#[derive(Debug)]
pub struct RevOrd<T: Ord> (pub T);

impl <T: Ord> Ord for RevOrd<T> {
    fn cmp(&self, other: &RevOrd<T>) -> Ordering {
        other.0.cmp(&self.0)        
    }
}

impl <T: Ord> PartialOrd for RevOrd<T> {
    fn partial_cmp(&self, other: &RevOrd<T>) -> Option<Ordering> {
        Some(other.0.cmp(&self.0))
    }
}

impl <T: Ord> PartialEq for RevOrd<T> {
    fn eq(&self, other: &RevOrd<T>) -> bool {
        self.0 == other.0
    }
}

impl <T: Ord> Eq for RevOrd<T> {}

impl <T: Ord> RevOrd<T> {
    fn unwrap(self) -> T {
        let RevOrd(item) = self;
        item
    }
}

#[derive(Debug)]
pub struct MinHeap<T: Ord> (BinaryHeap<RevOrd<T>>);

impl <T: Ord> MinHeap<T> {
    pub fn new() -> MinHeap<T> {
        MinHeap(BinaryHeap::new())
    }

    pub fn with_capacity(capacity: usize) -> MinHeap<T> {
        MinHeap(BinaryHeap::with_capacity(capacity))
    }

    pub fn peek(&self) -> Option<&T> {
        match self.0.peek() {
            Some(&RevOrd(ref item)) => Some(item),
            None => None,
        }
    }

    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    pub fn reserve_exact(&mut self, additional: usize) {
        self.0.reserve_exact(additional)
    }

    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional)
    }

    pub fn shrink_to_fit(&mut self) {
        self.0.shrink_to_fit()
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.0.pop() {
            Some(RevOrd(item)) => Some(item),
            None => None,
        }
    }

    pub fn push(&mut self, item: T) {
        self.0.push(RevOrd(item))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn append(&mut self, other: &mut MinHeap<T>) {
        self.0.append(&mut other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn RevOrd_Ord() {
        assert_eq!(RevOrd(1).cmp(&RevOrd(1)), Ordering::Equal);
        assert_eq!(RevOrd(1).cmp(&RevOrd(0)), Ordering::Less);
        assert_eq!(RevOrd(1).cmp(&RevOrd(2)), Ordering::Greater);
    }
    
    #[test]
    fn RevOrd_PartialOrd() {
        assert_eq!(RevOrd(1).partial_cmp(&RevOrd(1)), Some(Ordering::Equal));
        assert_eq!(RevOrd(1).partial_cmp(&RevOrd(0)), Some(Ordering::Less));
        assert_eq!(RevOrd(1).partial_cmp(&RevOrd(2)), Some(Ordering::Greater));
    }

    #[test]
    fn RevOrd_PartialEq() {
        assert_eq!(RevOrd(1) == RevOrd(1), true);
        assert_eq!(RevOrd(1) == RevOrd(0), false);
    }
    
    #[test]
    fn MinHeap_Constructors() {
        let mut m: MinHeap<usize> = MinHeap::new();
        let mut n: MinHeap<usize> = MinHeap::with_capacity(20);
        assert_eq!(m.len(), n.len());
    }
    
    #[test]
    fn MinHeap_test1() {
        let mut m = MinHeap::new();
        m.push(5);
        m.push(4);
        m.push(3);

        assert_eq!(m.pop().unwrap(), 3);
        assert_eq!(m.pop().unwrap(), 4);
        assert_eq!(m.pop().unwrap(), 5);
    }
}

