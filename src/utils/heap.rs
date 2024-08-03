#[derive(Clone)]
pub struct Heap<T: PartialOrd> {
    vec: Vec<T>,
}

impl<T: PartialOrd> Heap<T> {
    pub const fn new() -> Self {
        return Self { vec: Vec::new() };
    }

    pub fn with_capacity(capacity: usize) -> Self {
        return Self { vec: Vec::with_capacity(capacity) };
    }

    pub fn clear(&mut self) {
        self.vec.clear();
    }
    
    pub fn push(&mut self, item: T) {
        self.vec.push(item);
        self.sink_down(self.vec.len() - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
        let len = self.vec.len();
        if self.len() == 0 {
            return None;
        }

        self.vec.swap(0, len - 1);
        let res = self.vec.pop();
        self.sink_up(0);
        return res;
    }

    pub fn peek(&mut self) -> Option<&T> {
        return Some(&self.vec[0]);
    }

    pub fn len(&self) -> usize {
        return self.vec.len();
    }

    pub fn capacity(&self) -> usize {
        return self.vec.capacity();
    }

    /// Warning! Items will not come out ordered.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        return self.vec.iter();
    }

    pub fn into_ordered_vec(mut self) -> Vec<T> {
        let mut res = Vec::with_capacity(self.capacity());

        while let Some(x) = self.pop() {
            res.push(x);
        }

        return res;
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        let len = self.vec.len();
        if self.len() == 0 || index >= len {
            return None;
        }

        self.vec.swap(index, len - 1);
        let res = self.vec.pop();
        self.sink_up(index);
        return res;
    }

    fn sink_down(&mut self, mut index: usize) {
        while index > 0 {
            let parent = (index - 1) / 2;
            if self.vec[index] >= self.vec[parent] {
                break;
            }
            self.vec.swap(index, parent);

            index = parent;
        }
    }

    fn sink_up(&mut self, mut index: usize) {
        while index * 2 + 1 < self.vec.len() {
            let child_a = index * 2 + 1;
            let child_b = index * 2 + 2;

            let lower = if child_b < self.vec.len() && self.vec[child_b] < self.vec[child_a] { child_b } else { child_a };
            
            if self.vec[index] > self.vec[lower] {
                self.vec.swap(index, lower);
                index = lower;
                continue;
            }

            break;
        }
    }
}

#[cfg(test)]
mod test {
    use super::Heap;

    #[test]
    fn ord0() {
        let mut heap = Heap::new();
        heap.push(19u32);
        heap.push(3);
        heap.push(26);
        heap.push(5);
        heap.push(2);
        heap.push(69);
        heap.push(0);

        assert_eq!(heap.pop(), Some(0));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(19));
        assert_eq!(heap.pop(), Some(26));
        assert_eq!(heap.pop(), Some(69));
    }
}