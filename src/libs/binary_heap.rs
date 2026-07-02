pub type MinHeap<T> = Heap<T, { HeapType::Min as usize }>;
pub type MaxHeap<T> = Heap<T, { HeapType::Max as usize }>;

pub enum HeapType {
    Min = 0,
    Max = 1,
}

#[derive(Clone)]
pub struct Heap<T: Ord, const HT: usize> {
    items: Vec<T>,
}

impl<T: Ord, const HT: usize> Default for Heap<T, HT> {
    fn default() -> Self {
        Self { items: Vec::new() }
    }
}

impl<T: Ord, const HT: usize> Heap<T, HT> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn push(&mut self, item: T) {
        let mut i = self.items.len();
        self.items.push(item);
        while i != 0 {
            let p = (i - 1) / 2;
            if {
                if HT == HeapType::Min as usize {
                    self.items[p] <= self.items[i]
                } else {
                    self.items[p] >= self.items[i]
                }
            } {
                break;
            }
            self.items.swap(p, i);
            i = p;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.items.is_empty() {
            return None;
        }
        let result = self.items.swap_remove(0);
        self.sift_down(0);
        Some(result)
    }

    pub fn sift_down(&mut self, mut i: usize) {
        let len = self.items.len();
        while 2 * i + 1 < len {
            let mut j = 2 * i + 1;
            if j + 1 < len && {
                if HT == HeapType::Min as usize {
                    self.items[j] > self.items[j + 1]
                } else {
                    self.items[j] < self.items[j + 1]
                }
            } {
                j += 1;
            }
            if {
                if HT == HeapType::Min as usize {
                    self.items[i] <= self.items[j]
                } else {
                    self.items[i] >= self.items[j]
                }
            } {
                break;
            }
            self.items.swap(i, j);
            i = j;
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.get(0)
    }
}
