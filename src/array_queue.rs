#[derive(Debug)]
pub struct ArrayQueue<T> {
    a: Box<[Option<T>]>,
    j: usize,
    n: usize,
}

impl<T> ArrayQueue<T> {
    pub fn new(length: usize) -> Self {
        let a = Self::allocate(length);
        Self { a, j: 0, n: 0 }
    }

    pub fn length(&self) -> usize {
        self.a.len()
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        self.a.get((i + self.j) % self.a.len())?.as_ref()
    }

    pub fn set(&mut self, i: usize, x: T) -> Option<T> {
        self.a.get_mut((i + self.j) % self.a.len())?.replace(x)
    }

    pub fn add(&mut self, x: T) -> bool {
        if self.n + 1 > self.a.len() {
            self.resize();
        }
        self.a[(self.j + self.n) % self.a.len()] = Some(x);
        self.n += 1;
        true
    }

    pub fn remove(&mut self) -> Option<T> {
        let x = self.a.get_mut(self.j)?.take();
        self.j = (self.j + 1) % self.a.len();
        self.n -= 1;
        if self.a.len() >= 3 * self.n {
            self.resize();
        }
        x
    }

    fn allocate(n: usize) -> Box<[Option<T>]> {
        std::iter::repeat_with(|| None).take(n).collect()
    }

    fn resize(&mut self) {
        let mut b = Self::allocate(std::cmp::max(2 * self.n, 1));
        for k in 0..self.n {
            b[k] = self.a.get_mut((self.j + k) % self.a.len()).unwrap().take();
        }
        self.a = b;
        self.j = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup(queue: &mut ArrayQueue<char>, s: &str, offset: usize) {
        assert!(s.len() <= queue.length());
        for (i, c) in s.chars().enumerate() {
            queue.set((i + offset) % queue.length(), c);
        }
        queue.j = offset;
        queue.n = s.len();
    }

    fn check(queue: &ArrayQueue<char>, s: &str, offset: usize) {
        assert_eq!(queue.size(), s.chars().filter(|c| c != &'_').count());
        assert_eq!(queue.length(), s.len());
        assert_eq!(queue.j, offset);

        for (v, c) in std::iter::zip(&queue.a, s.chars()) {
            let expected = (c != '_').then_some(c);
            assert_eq!(v, &expected);
        }
    }

    #[test]
    fn test_setup_helper() {
        // i: 01234
        // a: _____
        let mut queue = ArrayQueue::new(5);

        // i: 01234
        // a: d_abc
        setup(&mut queue, "abcd", 2);

        assert_eq!(queue.j, 2);
        assert_eq!(queue.n, 4);

        let expected = vec![Some('d'), None, Some('a'), Some('b'), Some('c')].into_boxed_slice();
        assert_eq!(queue.a, expected);
    }

    #[test]
    fn test_check_helper() {
        let mut queue = ArrayQueue::new(5);
        setup(&mut queue, "abcd", 2);
        check(&queue, "d_abc", 2);
    }

    #[test]
    fn get() {
        let mut queue = ArrayQueue::new(6);
        setup(&mut queue, "abc", 2);
        check(&queue, "__abc_", 2);

        assert_eq!(queue.get(0), Some(&'a'));
        assert_eq!(queue.get(1), Some(&'b'));
        assert_eq!(queue.get(2), Some(&'c'));
        assert_eq!(queue.get(3), None);
        assert_eq!(queue.get(4), None);
        assert_eq!(queue.get(5), None);
        assert_eq!(queue.get(6), Some(&'a'));
        assert_eq!(queue.get(7), Some(&'b'));
        assert_eq!(queue.get(8), Some(&'c'));
        assert_eq!(queue.get(9), None);
        assert_eq!(queue.get(10), None);
        assert_eq!(queue.get(11), None);
    }

    #[test]
    fn set() {
        let mut queue = ArrayQueue::new(6);
        setup(&mut queue, "abc", 2);
        check(&queue, "__abc_", 2);

        queue.set(0, 'A');
        check(&queue, "__Abc_", 2);

        queue.set(6, 'B');
        check(&queue, "__Bbc_", 2);

        queue.set(12, 'C');
        check(&queue, "__Cbc_", 2);
    }

    #[test]
    fn resize_larger() {
        for j in 0..12 {
            let mut queue = ArrayQueue::new(6);
            setup(&mut queue, "abcdef", j);
            queue.resize();
            check(&queue, "abcdef______", 0);
        }
    }

    #[test]
    fn resize_smaller() {
        for j in 0..18 {
            let mut queue = ArrayQueue::new(9);
            setup(&mut queue, "abc", j);
            queue.resize();
            check(&queue, "abc___", 0);
        }
    }

    #[test]
    fn scenario() {
        let mut queue = ArrayQueue::new(6);
        check(&queue, "______", 0);

        setup(&mut queue, "abc", 2);
        check(&queue, "__abc_", 2);

        queue.add('d');
        check(&queue, "__abcd", 2);

        queue.add('e');
        check(&queue, "e_abcd", 2);

        assert_eq!(queue.remove(), Some('a'));
        check(&queue, "e__bcd", 3);

        queue.add('f');
        check(&queue, "ef_bcd", 3);

        queue.add('g');
        check(&queue, "efgbcd", 3);

        queue.add('h');
        check(&queue, "bcdefgh_____", 0);

        assert_eq!(queue.remove(), Some('b'));
        check(&queue, "_cdefgh_____", 1);
    }
}
