use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct SLList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    n: usize,
}

impl<T> Default for SLList<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct Node<T> {
    x: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(x: T) -> Self {
        Self { x, next: None }
    }
}

impl<T> SLList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            n: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn push(&mut self, x: T) {
        let u = Rc::new(RefCell::new(Node::new(x)));
        u.borrow_mut().next = self.head.take();
        if self.n == 0 {
            self.tail = Some(u.clone());
        }
        self.head = Some(u);
        self.n += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.n == 0 {
            return None;
        }

        let old_head = self.head.take().unwrap();
        let new_head = old_head.borrow_mut().next.take();

        self.head = new_head;

        self.n -= 1;

        if self.n == 0 {
            self.tail = None;
        }

        Rc::into_inner(old_head).map(|rc| rc.into_inner().x)
    }

    pub fn add(&mut self, x: T) {
        let u = Rc::new(RefCell::new(Node::new(x)));
        if self.n == 0 {
            self.head = Some(u.clone());
        } else {
            let old_tail = self.tail.take().unwrap();
            old_tail.borrow_mut().next = Some(u.clone());
        }

        self.tail = Some(u);
        self.n += 1;
    }

    pub fn remove(&mut self) -> Option<T> {
        if self.n == 0 {
            return None;
        }

        let old_head = self.head.take().unwrap();
        let new_head = old_head.borrow_mut().next.take();

        self.head = new_head;

        self.n -= 1;

        if self.n == 0 {
            self.tail = None;
        }

        Rc::into_inner(old_head).map(|v| v.into_inner().x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl<T> SLList<T> {
        fn get(&self, i: usize) -> Option<Rc<RefCell<Node<T>>>> {
            if self.n == 0 {
                return None;
            }

            if i == self.n - 1 {
                return self.tail.clone();
            }

            let mut cursor = self.head.clone();

            for _ in 0..i {
                cursor = cursor?.borrow().next.clone();
            }

            cursor.clone()
        }
    }

    fn check(list: &SLList<char>, expected: &str) {
        for (i, v) in expected.chars().enumerate() {
            assert_eq!(list.get(i).unwrap().borrow().x, v);
        }
        assert_eq!(list.size(), expected.len());
    }

    #[test]
    fn empty() {
        let list = SLList::<u8>::new();
        for i in 0..100 {
            assert!(list.get(i).is_none());
        }
    }

    #[test]
    fn push_to_empty() {
        let mut list = SLList::new();
        list.push('a');
        check(&list, "a");
    }

    #[test]
    fn add_to_empty() {
        let mut list = SLList::new();
        list.add('a');
        check(&list, "a");
    }

    #[test]
    fn remove_from_empty() {
        let mut list = SLList::<u8>::new();
        assert!(list.remove().is_none());
        assert!(list.head.is_none());
        assert!(list.tail.is_none());
    }

    #[test]
    fn pop_from_empty() {
        let mut list = SLList::<u8>::new();
        assert!(list.pop().is_none());
        assert!(list.head.is_none());
        assert!(list.tail.is_none());
    }

    #[test]
    fn remove_from_one() {
        let mut list = SLList::new();
        list.add(1);
        assert_eq!(list.remove(), Some(1));
        assert!(list.head.is_none());
        assert!(list.tail.is_none());
    }

    #[test]
    fn pop_from_one() {
        let mut list = SLList::new();
        list.add(1);
        assert_eq!(list.pop(), Some(1));
        assert!(list.head.is_none());
        assert!(list.tail.is_none());
    }

    #[test]
    fn scenario() {
        let mut list = SLList::new();
        let init = "abcde";
        for v in init.chars() {
            list.add(v);
        }
        check(&list, init);

        list.add('x');
        check(&list, "abcdex");

        assert_eq!(list.remove(), Some('a'));
        check(&list, "bcdex");

        assert_eq!(list.pop(), Some('b'));
        check(&list, "cdex");

        list.push('y');
        check(&list, "ycdex");
    }
}
