#[derive(Debug)]
pub struct ArrayStack<T> {
    a: Box<[Option<T>]>,
    n: usize,
}

impl<T> ArrayStack<T> {
    pub fn new(length: usize) -> Self {
        let a = Self::allocate(length);
        Self { a, n: 0 }
    }

    pub fn length(&self) -> usize {
        self.a.len()
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        self.a.get(i)?.as_ref()
    }

    pub fn set(&mut self, i: usize, x: T) -> Option<T> {
        self.a.get_mut(i)?.replace(x)
    }

    pub fn add(&mut self, i: usize, x: T) {
        if self.n + 1 > self.a.len() {
            self.resize();
        }
        for j in (i + 1..=self.n).rev() {
            self.a.swap(j, j - 1);
        }
        self.a[i] = Some(x);
        self.n += 1;
    }

    pub fn remove(&mut self, i: usize) -> Option<T> {
        let x = self.a.get_mut(i)?.take();
        for j in i..self.n - 1 {
            self.a.swap(j, j + 1);
        }
        self.n -= 1;
        if self.a.len() >= 3 * self.n {
            self.resize();
        }
        x
    }

    fn allocate(length: usize) -> Box<[Option<T>]> {
        std::iter::repeat_with(|| None).take(length).collect()
    }

    fn resize(&mut self) {
        let b = Self::allocate(std::cmp::max(2 * self.n, 1));
        let old_a = std::mem::replace(&mut self.a, b);
        for (i, v) in old_a.into_iter().enumerate().take(self.n) {
            self.a[i] = v;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(stack: &ArrayStack<char>, s: &str, length: usize) {
        assert_eq!(stack.size(), s.len());
        assert_eq!(stack.length(), length);

        for (i, c) in s.chars().enumerate() {
            assert_eq!(stack.get(i), Some(&c));
        }

        for i in s.len()..length {
            assert!(stack.get(i).is_none());
        }
    }

    #[test]
    fn scenario() {
        let mut stack = ArrayStack::<char>::new(6);
        check(&stack, "", 6);
        for (i, c) in "bred".chars().enumerate() {
            stack.add(i, c);
        }
        check(&stack, "bred", 6);
        stack.add(2, 'e');
        check(&stack, "breed", 6);
        stack.add(5, 'r');
        check(&stack, "breedr", 6);
        stack.add(5, 'e');
        check(&stack, "breeder", 12);
        stack.remove(4);
        check(&stack, "breeer", 12);
        stack.remove(4);
        check(&stack, "breer", 12);
        stack.remove(4);
        check(&stack, "bree", 8);
        stack.set(2, 'i');
        check(&stack, "brie", 8);
    }
}
