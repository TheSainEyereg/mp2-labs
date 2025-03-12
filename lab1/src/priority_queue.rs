pub struct PriorityQueue<T: Ord> {
    heap: Vec<T>,
}

impl<T: Ord> PriorityQueue<T> {
    pub fn new() -> Self {
        PriorityQueue { heap: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn push(&mut self, value: T) {
        self.heap.push(value);
        self.sift_up();
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.heap.is_empty() {
            return None;
        }

        let last = self.heap.len() - 1;
        self.heap.swap(0, last);

        let result = self.heap.pop();
        self.sift_down();

        result
    }

    pub fn peek(&self) -> Option<&T> {
        self.heap.first()
    }

    fn sift_up(&mut self) {
        let mut current = self.heap.len() - 1;
        while current > 0 {
            let parent = (current - 1) / 2;
            if self.heap[current] <= self.heap[parent] {
                break;
            }
            self.heap.swap(current, parent);
            current = parent;
        }
    }

    fn sift_down(&mut self) {
        let mut current = 0;
        loop {
            let left = 2 * current + 1;
            let right = 2 * current + 2;
            let mut largest = current;

            if left < self.heap.len() && self.heap[left] > self.heap[largest] {
                largest = left;
            }
            if right < self.heap.len() && self.heap[right] > self.heap[largest] {
                largest = right;
            }
            if largest == current {
                break;
            }
            self.heap.swap(current, largest);
            current = largest;
        }
    }
}
