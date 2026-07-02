use std::ptr::NonNull;

pub type MinHeap<T> = Heap<T, { HeapType::Min as usize }>;
pub type MaxHeap<T> = Heap<T, { HeapType::Max as usize }>;

pub enum HeapType {
    Min = 0,
    Max = 1,
}

#[derive(Clone)]
pub struct Heap<T: Ord, const HT: usize> {
    root: Option<NonNull<Node<T, HT>>>,
}

impl<T: Ord, const HT: usize> Drop for Heap<T, HT> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

impl<T: Ord, const HT: usize> Default for Heap<T, HT> {
    fn default() -> Self {
        Self { root: None }
    }
}

impl<T: Ord, const HT: usize> Heap<T, HT> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn push(&mut self, item: T) {
        unsafe {
            let node = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                item,
                left: None,
                right: None,
            })));
            self.root = Some(self.root.map_or(node, |root| Node::meld(root, node)));
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            let node = Box::from_raw(self.root?.as_ptr());
            self.root = node.left.map(|l| Node::meld_many(l));
            Some(node.item)
        }
    }

    pub fn peek(&self) -> Option<&T> {
        unsafe { Some(&(*self.root?.as_ptr()).item) }
    }

    pub fn append(&mut self, other: Self) {
        self.root = match (self.root.take(), other.root) {
            (a, None) => a,
            (None, b) => b,
            (Some(a), Some(b)) => Some(unsafe { Node::meld(a, b) }),
        };
        std::mem::forget(other);
    }

    pub fn collect(&self) -> Vec<&T> {
        let mut result = Vec::new();
        if let Some(x) = self.root {
            unsafe {
                let mut stack = vec![x];
                while let Some(node) = stack.pop() {
                    result.push(&node.as_ref().item);
                    if let Some(r) = node.as_ref().right {
                        stack.push(r);
                    }
                    if let Some(l) = node.as_ref().left {
                        stack.push(l);
                    }
                }
            }
        }
        result
    }
}

struct Node<T: Ord, const HT: usize> {
    item: T,
    left: Option<NonNull<Self>>,
    right: Option<NonNull<Self>>,
}

impl<T: Ord, const HT: usize> Node<T, HT> {
    unsafe fn meld(mut a: NonNull<Self>, mut b: NonNull<Self>) -> NonNull<Self> {
        if HT == HeapType::Min as usize {
            if a.as_ref().item > b.as_ref().item {
                std::mem::swap(&mut a, &mut b);
            }
        } else {
            if a.as_ref().item < b.as_ref().item {
                std::mem::swap(&mut a, &mut b);
            }
        }
        let c = (*a.as_ptr()).left;
        (*a.as_ptr()).left = Some(b);
        (*b.as_ptr()).right = c;
        a
    }

    unsafe fn meld_many(mut a: NonNull<Self>) -> NonNull<Self> {
        let mut right_spine = vec![a];
        while let Some(b) = a.as_ref().right {
            (*a.as_ptr()).right = None;
            a = b;
            right_spine.push(a);
        }
        for i in (0..right_spine.len() - 1).step_by(2) {
            right_spine[i] = Self::meld(right_spine[i], right_spine[i + 1]);
        }
        let mut i = (right_spine.len() - 1) & !1;
        while i != 0 {
            right_spine[i - 2] = Self::meld(right_spine[i - 2], right_spine[i]);
            i -= 2;
        }
        right_spine[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, Rng, SeedableRng};
    use std::{cmp::Reverse, collections::BinaryHeap};

    #[test]
    fn test() {
        let mut rng = StdRng::seed_from_u64(42);
        for testcase_id in 0..20 {
            eprintln!("Case #{testcase_id}");
            let mut pairing = MinHeap::new();
            let mut binary = BinaryHeap::new();
            for _ in 0..20 {
                match rng.gen_range(0..2) {
                    0 => {
                        let x = rng.gen_range(0..32);
                        eprintln!("Push {x}");
                        pairing.push(x);
                        binary.push(Reverse(x));
                    }
                    1 => {
                        let result = pairing.pop();
                        let expected = binary.pop().map(|x| x.0);
                        eprintln!("Poped {result:?} vs {expected:?}");
                        assert_eq!(result, expected);
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}
