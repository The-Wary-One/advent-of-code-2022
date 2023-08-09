#[derive(Debug)]
pub struct MinHeap<T> {
    data: Vec<T>,
}

impl<T: Ord> MinHeap<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    #[allow(dead_code)]
    pub fn with_capacity(n: usize) -> Self {
        Self {
            data: Vec::with_capacity(n),
        }
    }

    pub fn push(&mut self, item: T) {
        let len = self.data.len();
        self.data.push(item);
        self.heapify_up(len);
    }

    pub fn pop(&mut self) -> Option<T> {
        let len = self.data.len();
        if len > 1 {
            self.data.swap(0, len - 1);
        }

        let oi = self.data.pop();
        self.heapify_down(0);
        oi
    }

    fn parent_idx(idx: usize) -> usize {
        (idx - 1) / 2
    }

    fn left_child_idx(idx: usize) -> usize {
        2 * idx + 1
    }

    fn right_child_idx(idx: usize) -> usize {
        2 * idx + 2
    }

    fn heapify_up(&mut self, idx: usize) {
        if idx == 0 {
            return;
        }

        let p_idx = Self::parent_idx(idx);
        let p_value = &self.data[p_idx];
        let curr_value = &self.data[idx];

        if curr_value < p_value {
            self.data.swap(idx, p_idx);
            self.heapify_up(p_idx);
        }
    }

    fn heapify_down(&mut self, idx: usize) {
        if idx >= self.data.len() {
            return;
        }

        let curr_value = &self.data[idx];
        let left_child_idx = Self::left_child_idx(idx);
        let left_child_value = match self.data.get(left_child_idx) {
            Some(item) => item,
            None => return,
        };
        let right_child_idx = Self::right_child_idx(idx);
        let right_child_value = self.data.get(right_child_idx);

        let (target_idx, target_value) = match (left_child_value, right_child_value) {
            (left, Some(right)) if right < left => (right_child_idx, right),
            _ => (left_child_idx, left_child_value),
        };
        if curr_value > target_value {
            self.data.swap(idx, target_idx);
            self.heapify_down(target_idx);
        }
    }
}

impl<T> MinHeap<T> {
    #[allow(dead_code)]
    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }
}

impl<T: Ord, const N: usize> From<[T; N]> for MinHeap<T> {
    fn from(value: [T; N]) -> Self {
        let mut data = Vec::from(value);
        data.sort();
        Self { data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_peek() {
        let mut heap = MinHeap::new();

        heap.push(2);
        heap.push(3);
        heap.push(1);

        assert_eq!(heap.peek(), Some(&1));
    }

    #[test]
    fn test_pop() {
        let mut heap = MinHeap::from([3, 1]);

        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), None);
    }
}
