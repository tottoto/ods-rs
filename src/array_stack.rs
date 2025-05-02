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
        unsafe {
            std::ptr::copy(
                self.a[i..self.n].as_ptr(),
                self.a[i + 1..self.n + 1].as_mut_ptr(),
                self.n - i,
            )
        }
        self.a[i] = Some(x);
        self.n += 1;
    }

    pub fn remove(&mut self, i: usize) -> Option<T> {
        let x = self.a.get_mut(i)?.take();
        unsafe {
            std::ptr::copy(
                self.a[i + 1..self.n].as_ptr(),
                self.a[i..self.n - 1].as_mut_ptr(),
                self.n - i,
            );
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
        let mut b = Self::allocate(std::cmp::max(2 * self.n, 1));
        unsafe {
            std::ptr::copy(self.a.as_ptr(), b.as_mut_ptr(), self.n);
        }
        self.a = b;
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
