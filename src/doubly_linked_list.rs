use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct DLList<T> {
    dummy_head: Rc<RefCell<Node<T>>>,
    dummy_tail: Rc<RefCell<Node<T>>>,
    n: usize,
}

#[derive(Debug)]
pub struct Node<T> {
    x: Option<T>,
    prev: Option<Weak<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn dummy() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            x: None,
            prev: None,
            next: None,
        }))
    }

    fn new(x: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            x: Some(x),
            prev: None,
            next: None,
        }))
    }
}

impl<T> Default for DLList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> DLList<T> {
    pub fn new() -> Self {
        let dummy_head = Node::dummy();
        let dummy_tail = Node::dummy();
        dummy_head.borrow_mut().next = Some(Rc::clone(&dummy_tail));
        dummy_tail.borrow_mut().prev = Some(Rc::downgrade(&dummy_head));
        Self {
            dummy_head,
            dummy_tail,
            n: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn get_node(&self, i: usize) -> Option<Rc<RefCell<Node<T>>>> {
        if i > self.n {
            return None;
        }

        let mut p;
        if i < self.n / 2 {
            p = self.dummy_head.borrow().next.clone().unwrap();
            for _ in 0..i {
                p = p.clone().borrow().next.clone().unwrap();
            }
        } else {
            p = self.dummy_tail.clone();
            for _ in i..self.n {
                p = p.clone().borrow().prev.as_ref().unwrap().upgrade().unwrap();
            }
        }
        Some(p)
    }

    pub fn get(&self, i: usize) -> Option<T>
    where
        T: Clone,
    {
        self.get_node(i)?.borrow().x.clone()
    }

    pub fn set(&self, i: usize, x: T) -> Option<T> {
        self.get_node(i)?.borrow_mut().x.replace(x)
    }

    pub fn add_before(
        &mut self,
        w: Option<Rc<RefCell<Node<T>>>>,
        x: T,
    ) -> Option<Rc<RefCell<Node<T>>>> {
        let w = w?;
        let u = Node::new(x);
        let v = w.borrow_mut().prev.take().unwrap().upgrade().unwrap();
        v.borrow_mut().next = Some(u.clone());
        u.borrow_mut().prev = Some(Rc::downgrade(&v));
        w.borrow_mut().prev = Some(Rc::downgrade(&u));
        u.borrow_mut().next = Some(w);
        self.n += 1;
        Some(u)
    }

    pub fn add(&mut self, i: usize, x: T) -> bool {
        self.add_before(self.get_node(i), x).is_some()
    }

    pub fn remove_node(&mut self, w: Option<Rc<RefCell<Node<T>>>>) {
        assert!(self.n > 0);

        let Some(w) = w else {
            return;
        };

        let p = w.borrow_mut().prev.take().unwrap().upgrade().unwrap();
        let n = w.borrow_mut().next.take().unwrap();

        n.borrow_mut().prev = Some(Rc::downgrade(&p));
        p.borrow_mut().next = Some(n);

        self.n -= 1;
    }

    pub fn remove(&mut self, i: usize) -> Option<T> {
        let w = self.get_node(i)?;
        let x = w.borrow_mut().x.take()?;
        self.remove_node(Some(w));
        Some(x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(list: &DLList<char>, expected: &str) {
        for (i, v) in expected.chars().enumerate() {
            assert_eq!(list.get(i), Some(v));
        }
        assert_eq!(list.size(), expected.len());
    }

    fn setup(list: &mut DLList<char>, s: &str) {
        for (i, v) in s.chars().enumerate() {
            assert!(list.add(i, v))
        }
    }

    #[test]
    fn test_setup_helper() {
        let mut list = DLList::new();
        let s = "abcdefg";
        setup(&mut list, s);
        check(&list, s);
    }

    #[test]
    fn scenario() {
        let mut list = DLList::new();
        setup(&mut list, "abcde");

        assert!(list.add(0, 'x'));
        check(&list, "xabcde");

        assert!(list.add(1, 'y'));
        check(&list, "xyabcde");

        assert!(list.add(7, 'B'));
        check(&list, "xyabcdeB");

        assert!(list.add(7, 'A'));
        check(&list, "xyabcdeAB");

        assert_eq!(list.remove(2), Some('a'));
        check(&list, "xybcdeAB");

        assert_eq!(list.remove(0), Some('x'));
        check(&list, "ybcdeAB");

        assert_eq!(list.remove(6), Some('B'));
        check(&list, "ybcdeA");

        assert_eq!(list.remove(4), Some('e'));
        check(&list, "ybcdA");
    }

    #[test]
    fn remove_from_empty() {
        let mut list = DLList::new();
        for i in 0..100 {
            assert!(list.remove(i).is_none());
            check(&list, "");
        }
    }

    #[test]
    fn remove_from_out_of_bound() {
        let mut list = DLList::new();
        let initial = "abcde";
        setup(&mut list, initial);

        for i in initial.len()..100 {
            assert!(list.remove(i).is_none());
        }
    }

    #[test]
    fn remove_from_one() {
        let mut list = DLList::new();
        setup(&mut list, "a");

        assert_eq!(list.remove(0), Some('a'));
        check(&list, "");
        assert!(
            list.dummy_head
                .borrow()
                .next
                .as_ref()
                .unwrap()
                .borrow()
                .next
                .is_none()
        );
        assert!(
            list.dummy_tail
                .borrow()
                .prev
                .as_ref()
                .unwrap()
                .upgrade()
                .unwrap()
                .borrow()
                .prev
                .is_none()
        );
    }

    #[test]
    fn add_to_out_of_bound() {
        let mut list = DLList::new();
        let initial = "abcde";
        setup(&mut list, "abcde");

        for i in initial.len() + 1..100 {
            assert!(!list.add(i, 'Z'));
        }
    }

    #[test]
    #[should_panic(expected = "already borrowed: BorrowMutError")]
    fn add_before_with_node_borrowed_as_mut_should_panic() {
        let mut list = DLList::new();
        let node = list.get_node(0).unwrap();
        let node2 = node.clone();
        let _mut_node = node2.borrow_mut();
        list.add_before(Some(node), 'x');
    }
}
