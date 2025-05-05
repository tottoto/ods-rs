use crate::ArrayStack;

#[derive(Debug)]
pub struct RootishArrayStack<T> {
    blocks: ArrayStack<Box<[Option<T>]>>,
    n: usize,
}

fn i2b(i: usize) -> usize {
    ((-3. + (9. + 8. * i as f64).sqrt()) / 2.).ceil() as usize
}

impl<T> RootishArrayStack<T> {
    pub fn new(min_length: usize) -> Self {
        let blocks = if min_length == 0 {
            ArrayStack::new(0)
        } else {
            let r = i2b(min_length - 1) + 1;
            let mut stack = ArrayStack::new(r);
            for b in 0..r {
                stack.add(b, crate::util::allocate(b + 1));
            }
            stack
        };
        Self { blocks, n: 0 }
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn length(&self) -> usize {
        let mut length = 0;
        for i in 0..self.blocks.size() {
            length += self.blocks.get(i).unwrap().len();
        }
        length
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        let b = i2b(i);
        let j = i - b * (b + 1) / 2;
        self.blocks.get(b)?.get(j)?.as_ref()
    }

    pub(crate) fn take(&mut self, i: usize) -> Option<T> {
        let b = i2b(i);
        let j = i - b * (b + 1) / 2;
        self.blocks.get_mut(b)?.get_mut(j)?.take()
    }

    pub fn set(&mut self, i: usize, x: T) -> Option<T> {
        let b = i2b(i);
        let j = i - b * (b + 1) / 2;
        self.blocks.get_mut(b)?.get_mut(j)?.replace(x)
    }

    pub fn add(&mut self, i: usize, x: T) {
        let r = self.blocks.size();
        if r * (r + 1) / 2 < self.n + 1 {
            self.grow()
        }
        self.n += 1;
        for j in (i + 1..self.n).rev() {
            let tmp = self.take(j - 1).unwrap();
            self.set(j, tmp);
        }
        self.set(i, x);
    }

    fn grow(&mut self) {
        let block = crate::util::allocate(self.blocks.size() + 1);
        self.blocks.add(self.blocks.size(), block);
    }

    pub fn remove(&mut self, i: usize) -> Option<T> {
        let x = self.take(i);
        for j in i..self.n - 1 {
            let tmp = self.take(j + 1).unwrap();
            self.set(j, tmp);
        }
        self.n -= 1;
        let r = self.blocks.size();
        if (r - 2) * (r - 1) / 2 >= self.n {
            self.shrink();
        }
        x
    }

    fn shrink(&mut self) {
        let mut r = self.blocks.size();
        while r > 0 && (r - 2) * (r - 1) / 2 >= self.n {
            self.blocks.remove(self.blocks.size() - 1);
            r -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn i2b_successful() {
        let mut i = 0;
        for b in 0..=100 {
            for _ in 0..b + 1 {
                assert_eq!(i2b(i), b);
                i += 1;
            }
        }
    }

    #[test]
    fn constructor() {
        for (min_length, blocks_length, stack_length) in [
            (0, 0, 0),
            (1, 1, 1),
            (2, 2, 3),
            (3, 2, 3),
            (4, 3, 6),
            (5, 3, 6),
            (6, 3, 6),
            (7, 4, 10),
        ] {
            let stack = RootishArrayStack::<bool>::new(min_length);
            assert_eq!(stack.blocks.length(), blocks_length);
            assert_eq!(stack.length(), stack_length);
            assert_eq!(stack.size(), 0);
        }
    }

    fn check(stack: &RootishArrayStack<char>, expected: &str) {
        if expected.is_empty() {
            assert_eq!(stack.size(), 0);
            return;
        }

        for (i, v) in expected.chars().enumerate() {
            assert_eq!(stack.get(i), Some(&v));
        }

        let mut count = 0;
        let mut start = 0;

        const MAX_LOOP: usize = 100;

        for b in 0..MAX_LOOP {
            let end = (start + b + 1).min(expected.len());

            let block = stack.blocks.get(b).unwrap();
            for (i, v) in expected[start..end].chars().enumerate() {
                assert_eq!(block[i], Some(v));
                count += 1;
            }

            if end == expected.len() {
                break;
            }

            start = end;
        }

        // All alements have been checked.
        assert_eq!(stack.size(), expected.len());
        assert_eq!(stack.size(), count);
    }

    #[test]
    fn layout() {
        let a0 = vec![Some('a')].into_boxed_slice();
        let a1 = vec![Some('b'), Some('c')].into_boxed_slice();
        let a2 = vec![Some('d'), Some('e'), Some('f')].into_boxed_slice();
        let a3 = vec![Some('g'), Some('h'), None, None].into_boxed_slice();

        let a = vec![Some(a0), Some(a1), Some(a2), Some(a3)].into_boxed_slice();
        let blocks = ArrayStack::from_raw(a, 4);

        let stack = RootishArrayStack { blocks, n: 8 };

        check(&stack, "abcdefgh");
        assert_eq!(stack.length(), 10);
    }

    #[test]
    fn add() {
        let mut stack = RootishArrayStack::new(8);
        for (i, v) in "abcdefgh".chars().enumerate() {
            stack.add(i, v);
        }
        check(&stack, "abcdefgh");
        stack.add(1, 'x');
        check(&stack, "axbcdefgh");
    }

    #[test]
    fn grow() {
        let mut stack = RootishArrayStack::new(0);
        assert_eq!(stack.blocks.size(), 0);

        stack.add(0, 'a');
        check(&stack, "a");
        assert_eq!(stack.blocks.size(), 1);

        stack.add(0, 'b');
        check(&stack, "ba");
        assert_eq!(stack.blocks.size(), 2);

        stack.add(0, 'c');
        check(&stack, "cba");
        assert_eq!(stack.blocks.size(), 2);

        stack.add(0, 'd');
        check(&stack, "dcba");
        assert_eq!(stack.blocks.size(), 3);

        stack.add(0, 'e');
        check(&stack, "edcba");
        assert_eq!(stack.blocks.size(), 3);

        stack.add(0, 'f');
        check(&stack, "fedcba");
        assert_eq!(stack.blocks.size(), 3);

        stack.add(0, 'g');
        check(&stack, "gfedcba");
        assert_eq!(stack.blocks.size(), 4);
    }

    #[test]
    fn shrink() {
        let mut stack = RootishArrayStack::new(11);
        for (i, v) in "abcdefgh".chars().enumerate() {
            stack.add(i, v);
        }
        check(&stack, "abcdefgh");
        assert_eq!(stack.blocks.size(), 5);

        stack.remove(0);
        check(&stack, "bcdefgh");
        assert_eq!(stack.blocks.size(), 5);

        stack.remove(0);
        check(&stack, "cdefgh");
        assert_eq!(stack.blocks.size(), 4);

        stack.remove(0);
        check(&stack, "defgh");
        assert_eq!(stack.blocks.size(), 4);

        stack.remove(0);
        check(&stack, "efgh");
        assert_eq!(stack.blocks.size(), 4);

        stack.remove(0);
        check(&stack, "fgh");
        assert_eq!(stack.blocks.size(), 3);
    }

    #[test]
    fn scenario() {
        let mut stack = RootishArrayStack::new(0);

        let mut expected = String::new();
        check(&stack, &expected);
        for (i, v) in "abcdefgh".chars().enumerate() {
            stack.add(i, v);
            expected.push(v);
            check(&stack, &expected);
        }

        assert_eq!(expected, "abcdefgh");
        check(&stack, "abcdefgh");

        stack.add(2, 'x');
        check(&stack, "abxcdefgh");

        assert_eq!(stack.remove(1), Some('b'));
        check(&stack, "axcdefgh");

        assert_eq!(stack.remove(7), Some('h'));
        check(&stack, "axcdefg");

        assert_eq!(stack.remove(6), Some('g'));
        check(&stack, "axcdef");
    }
}
