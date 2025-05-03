#[derive(Debug)]
pub struct ArrayDeque<T> {
    a: Box<[Option<T>]>,
    j: usize,
    n: usize,
}

impl<T> ArrayDeque<T> {
    pub fn new(length: usize) -> Self {
        let a = crate::util::allocate(length);
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

    pub fn add(&mut self, i: usize, x: T) {
        if self.n + 1 > self.a.len() {
            self.resize();
        }

        if i < self.n.div_ceil(2) {
            // shift left part to left
            self.j = if self.j == 0 {
                self.a.len() - 1
            } else {
                self.j - 1
            };

            for k in 0..i {
                self.a
                    .swap((self.j + k) % self.a.len(), (self.j + k + 1) % self.a.len());
            }
        } else {
            // shift right part to right
            for k in (i + 1..=self.n).rev() {
                self.a
                    .swap((self.j + k) % self.a.len(), (self.j + k - 1) % self.a.len());
            }
        }

        self.a[(self.j + i) % self.a.len()] = Some(x);
        self.n += 1;
    }

    pub fn remove(&mut self, i: usize) -> Option<T> {
        let x = self.a.get_mut((self.j + i) % self.a.len())?.take();

        if i < self.n.div_ceil(2) {
            // shift left part to right
            for k in (1..=i).rev() {
                self.a
                    .swap((self.j + k) % self.a.len(), (self.j + k - 1) % self.a.len());
            }
            self.j = (self.j + 1) % self.a.len();
        } else {
            // shift right part to left
            for k in i..self.n - 1 {
                self.a
                    .swap((self.j + k) % self.a.len(), (self.j + k + 1) % self.a.len());
            }
        }

        self.n -= 1;
        if self.a.len() >= 3 * self.n {
            self.resize();
        }
        x
    }

    fn resize(&mut self) {
        let mut b = crate::util::allocate(std::cmp::max(2 * self.n, 1));
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

    fn setup(deque: &mut ArrayDeque<char>, s: &str, offset: usize) {
        assert!(s.len() <= deque.length());
        for (i, c) in s.chars().enumerate() {
            deque.set((i + offset) % deque.length(), c);
        }
        deque.j = offset;
        deque.n = s.len();
    }

    fn check(deque: &ArrayDeque<char>, s: &str, offset: usize) {
        assert_eq!(deque.size(), s.chars().filter(|c| c != &'_').count());
        assert_eq!(deque.length(), s.len());
        assert_eq!(deque.j, offset);

        for (v, c) in std::iter::zip(&deque.a, s.chars()) {
            let expected = (c != '_').then_some(c);
            assert_eq!(v, &expected);
        }
    }

    #[test]
    fn test_setup_helper() {
        // i: 01234
        // a: _____
        let mut deque = ArrayDeque::new(5);

        // i: 01234
        // a: d_abc
        setup(&mut deque, "abcd", 2);

        assert_eq!(deque.j, 2);
        assert_eq!(deque.n, 4);

        let expected = vec![Some('d'), None, Some('a'), Some('b'), Some('c')].into_boxed_slice();
        assert_eq!(deque.a, expected);
    }

    #[test]
    fn test_check_helper() {
        let mut deque = ArrayDeque::new(5);
        setup(&mut deque, "abcd", 2);
        check(&deque, "d_abc", 2);
    }

    #[test]
    fn get() {
        let mut deque = ArrayDeque::new(6);
        setup(&mut deque, "abc", 2);
        check(&deque, "__abc_", 2);

        assert_eq!(deque.get(0), Some(&'a'));
        assert_eq!(deque.get(1), Some(&'b'));
        assert_eq!(deque.get(2), Some(&'c'));
        assert_eq!(deque.get(3), None);
        assert_eq!(deque.get(4), None);
        assert_eq!(deque.get(5), None);
        assert_eq!(deque.get(6), Some(&'a'));
        assert_eq!(deque.get(7), Some(&'b'));
        assert_eq!(deque.get(8), Some(&'c'));
        assert_eq!(deque.get(9), None);
        assert_eq!(deque.get(10), None);
        assert_eq!(deque.get(11), None);
    }

    #[test]
    fn set() {
        let mut deque = ArrayDeque::new(6);
        setup(&mut deque, "abc", 2);
        check(&deque, "__abc_", 2);

        deque.set(0, 'A');
        check(&deque, "__Abc_", 2);

        deque.set(6, 'B');
        check(&deque, "__Bbc_", 2);

        deque.set(12, 'C');
        check(&deque, "__Cbc_", 2);
    }

    #[test]
    fn resize_larger() {
        for j in 0..12 {
            let mut deque = ArrayDeque::new(6);
            setup(&mut deque, "abcdef", j);
            deque.resize();
            check(&deque, "abcdef______", 0);
        }
    }

    #[test]
    fn resize_smaller() {
        for j in 0..18 {
            let mut deque = ArrayDeque::new(9);
            setup(&mut deque, "abc", j);
            deque.resize();
            check(&deque, "abc___", 0);
        }
    }

    #[test]
    fn scenario() {
        let mut deque = ArrayDeque::new(12);
        check(&deque, "____________", 0);

        setup(&mut deque, "abcdefgh", 0);
        check(&deque, "abcdefgh____", 0);

        assert_eq!(deque.remove(2), Some('c'));
        check(&deque, "_abdefgh____", 1);

        deque.add(4, 'x');
        check(&deque, "_abdexfgh___", 1);

        deque.add(3, 'y');
        check(&deque, "abdyexfgh___", 0);

        deque.add(4, 'z');
        check(&deque, "bdyzexfgh__a", 11);

        deque.add(1, 'A');
        check(&deque, "bdyzexfgh_aA", 10);

        deque.add(2, 'B');
        check(&deque, "bdyzexfghaAB", 9);

        deque.add(3, 'C');
        check(&deque, "ABCbdyzexfgh___________a", 23);

        assert_eq!(deque.remove(0), Some('a'));
        check(&deque, "ABCbdyzexfgh____________", 0);
    }
}
