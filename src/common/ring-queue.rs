use common::Queue;
use core::ptr::read_volatile;

// pub enum IterMix<Q, A, B, C> 
//     where A: Iterator<Item = Q>,
//           B: Iterator<Item = Q>,
//           C: Iterator<Item = Q>
// {
//     A(A),
//     B(B),
//     C(C)
// } 

// impl<Q, A, B, C> Iterator for IterMix<Q, A, B, C> 
//     where A: Iterator<Item = Q>,
//           B: Iterator<Item = Q>,
//           C: Iterator<Item = Q>
// {
//     type Item = Q;

//     fn next(&mut self) -> Self::Item {
//         match *self {
//             IterMix::A(ref mut val) => val.next(),
//             IterMix::B(ref mut val) => val.next(),
//             IterMix::C(ref mut val) => val.next(),
//         }
//     }
// } 

pub struct InterruptQueue {
    head: usize,
    tail: usize,
    ring: [u8; 128]
}

impl InterruptQueue{
    pub const fn new() -> InterruptQueue {
        InterruptQueue {
            head: 0,
            tail: 0,
            ring: [0; 128],
        }
    }
}

impl Queue<u8> for InterruptQueue {
    // type Iter = IterMix<u8>;

    fn has_elements(&self) -> bool {
        unsafe {
            let head = read_volatile(&self.head);
            let tail = read_volatile(&self.tail);
            head != tail
        }
    }

    fn is_full(&self) -> bool {
        unsafe { read_volatile(&self.head) == ((read_volatile(&self.tail) + 1) % self.ring.len()) }
    }

    fn len(&self) -> usize {
        let head = unsafe { read_volatile(&self.head) };
        let tail = unsafe { read_volatile(&self.tail) };

        if tail > head {
            tail - head
        } else if tail < head {
            (self.ring.len() - head) + tail
        } else {
            // head equals tail, length is zero
            0
        }
    }

    fn enqueue(&mut self, val: u8) -> bool {
        unsafe {
            let head = read_volatile(&self.head);
            if ((self.tail + 1) % self.ring.len()) == head {
                // Incrementing tail will overwrite head
                return false;
            } else {
                self.ring[self.tail] = val;
                self.tail = (self.tail + 1) % self.ring.len();
                return true;
            }
        }
    }

    fn dequeue(&mut self) -> Option<u8> {
        if self.has_elements() {
            let val = self.ring[self.head];
            self.head = (self.head + 1) % self.ring.len();
            Some(val)
        } else {
            None
        }
    }

    fn peek(&self) -> Option<u8> {
        if self.has_elements() {
            Some(self.ring[self.head])
        } else {
            None
        }
    }

    // fn iter(&self) -> Self::Iter {
    //     let head = unsafe { read_volatile(&self.head) };
    //     let tail = unsafe { read_volatile(&self.tail) };

    //     if tail > head {
    //         IterMix::A(self.ring[head .. tail])
    //     } else if tail < head {
    //         IterMix::B(self.ring[head .. self.ring.len()].chain(self.ring[0 .. tail]))
    //     } else {
    //         IterMix::C(iter::empty())
    //     }
    // }

    fn take(&mut self, val: u8) -> Option<u8> {
        let head = unsafe { read_volatile(&self.head) };
        let tail = unsafe { read_volatile(&self.tail) };

        if head != tail && self.ring[self.head] == val {
            let tmp = self.ring[self.head];
            self.head = (self.head + 1) % self.ring.len();
            Some(tmp)
        } else {
            None
        }
    }
}
