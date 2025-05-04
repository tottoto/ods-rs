use crate::ArrayStack;

#[derive(Debug)]
pub struct DualArrayDeque<T> {
    front: ArrayStack<T>,
    back: ArrayStack<T>,
}

impl<T> DualArrayDeque<T> {
    pub fn new(length: usize) -> Self {
        let nf = length / 2;
        let nb = length - nf;

        Self {
            front: ArrayStack::new(nf),
            back: ArrayStack::new(nb),
        }
    }

    pub fn length(&self) -> usize {
        self.front.length() + self.back.length()
    }

    pub fn size(&self) -> usize {
        self.front.size() + self.back.size()
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        if i < self.front.size() {
            self.front.get(self.front.size() - i - 1)
        } else {
            self.back.get(i - self.front.size())
        }
    }

    pub(crate) fn take(&mut self, i: usize) -> Option<T> {
        if i < self.front.size() {
            self.front.take(self.front.size() - i - 1)
        } else {
            self.back.take(i - self.front.size())
        }
    }

    pub fn set(&mut self, i: usize, x: T) {
        if i < self.front.size() {
            self.front.set(self.front.size() - i - 1, x);
        } else {
            self.back.set(i - self.front.size(), x);
        }
    }

    pub fn add(&mut self, i: usize, x: T) {
        if i < self.front.size() {
            self.front.add(self.front.size() - i, x);
        } else {
            self.back.add(i - self.front.size(), x);
        }
        self.balance();
    }

    pub fn remove(&mut self, i: usize) -> Option<T> {
        let x = if i < self.front.size() {
            self.front.remove(self.front.size() - i - 1)
        } else {
            self.back.remove(i - self.front.size())
        };
        self.balance();
        x
    }

    fn balance(&mut self) {
        if !(3 * self.front.size() < self.back.size() || 3 * self.back.size() < self.front.size()) {
            return;
        }

        let n = self.size();

        let nf = n / 2;
        let mut af = crate::util::allocate(std::cmp::max(2 * nf, 1));
        for i in 0..nf {
            af[nf - i - 1] = self.take(i);
        }

        let nb = n - nf;
        let mut ab = crate::util::allocate(std::cmp::max(2 * nb, 1));
        for i in 0..nb {
            ab[i] = self.take(nf + i);
        }

        self.front = ArrayStack::from_raw(af, nf);
        self.back = ArrayStack::from_raw(ab, nb);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length() {
        let deque = DualArrayDeque::<i32>::new(9);

        assert_eq!(deque.length(), 9);
        assert_eq!(deque.front.length(), 4);
        assert_eq!(deque.back.length(), 5);
    }

    #[test]
    fn layout() {
        let mut front = ArrayStack::new(5);
        front.add(0, 'b');
        front.add(1, 'a');

        let mut back = ArrayStack::new(5);
        back.add(0, 'c');
        back.add(1, 'd');

        // front|back
        // 43210|01234
        // ___ab|cd___
        let deque = DualArrayDeque { front, back };

        assert_eq!(deque.length(), 10);
        assert_eq!(deque.size(), 4);

        for (i, v) in "abcd".chars().enumerate() {
            assert_eq!(deque.get(i), Some(&v))
        }
    }

    fn create(
        front_content: &str,
        front_length: usize,
        back_content: &str,
        back_length: usize,
    ) -> DualArrayDeque<char> {
        assert!(front_content.len() <= front_length);
        assert!(back_content.len() <= back_length);

        let mut front = ArrayStack::new(front_length);
        for (i, v) in front_content.chars().rev().enumerate() {
            front.add(i, v);
        }

        let mut back = ArrayStack::new(back_length);
        for (i, v) in back_content.chars().enumerate() {
            back.add(i, v);
        }

        DualArrayDeque { front, back }
    }

    #[test]
    fn test_create_helper() {
        let deque = create("ab", 5, "cd", 5);

        assert_eq!(deque.length(), 10);
        assert_eq!(deque.size(), 4);

        for (i, v) in "abcd".chars().enumerate() {
            assert_eq!(deque.get(i), Some(&v))
        }
    }

    fn check(
        deque: &DualArrayDeque<char>,
        front_content: &str,
        front_length: usize,
        back_content: &str,
        back_length: usize,
    ) {
        assert_eq!(deque.front.length(), front_length);
        assert_eq!(deque.front.size(), front_content.len());
        for (i, v) in front_content.chars().rev().enumerate() {
            assert_eq!(deque.front.get(i), Some(&v));
        }

        assert_eq!(deque.back.length(), back_length);
        assert_eq!(deque.back.size(), back_content.len());
        for (i, v) in back_content.chars().enumerate() {
            assert_eq!(deque.back.get(i), Some(&v));
        }

        assert_eq!(deque.length(), front_length + back_length);
        assert_eq!(deque.size(), front_content.len() + back_content.len());
        for (i, v) in format!("{front_content}{back_content}").chars().enumerate() {
            assert_eq!(deque.get(i), Some(&v));
        }
    }

    #[test]
    fn test_check_helper() {
        let deque = create("ab", 5, "cd", 5);
        check(&deque, "ab", 5, "cd", 5);
    }

    #[test]
    fn balance_to_front() {
        let mut deque = create("a", 5, "bcde", 5);
        deque.balance();
        check(&deque, "ab", 4, "cde", 6);
    }

    #[test]
    fn balance_to_back() {
        let mut deque = create("abcd", 5, "e", 5);
        deque.balance();
        check(&deque, "ab", 4, "cde", 6);
    }

    #[test]
    fn scenario() {
        let mut deque = create("ab", 5, "cd", 5);

        deque.add(3, 'x');
        check(&deque, "ab", 5, "cxd", 5);

        deque.add(4, 'y');
        check(&deque, "ab", 5, "cxyd", 5);

        deque.remove(0);
        check(&deque, "bc", 4, "xyd", 6);
    }
}
