pub trait Queue<T> {
    // type Iter: Iterator<Item = T>;

    fn has_elements(&self) -> bool;
    fn is_full(&self) -> bool;
    fn len(&self) -> usize;
    fn enqueue(&mut self, val: T) -> bool;
    fn dequeue(&mut self) -> Option<T>;
    fn peek(&self) -> Option<T>;
    // fn iter(&self) -> Self::Iter;
    fn take(&mut self, val: T) -> Option<T>;
}