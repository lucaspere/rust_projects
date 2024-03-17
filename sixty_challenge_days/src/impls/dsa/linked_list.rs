use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

/// A [LinkedList]
/// fdf
///
///
/// fdfd
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
/// fdfd
#[derive(Debug)]
struct LinkedList<'a, T> {
    cursor: usize,
    len: usize,
    head: Option<Rc<RefCell<Node<&'a T>>>>,
    tail: Option<Rc<RefCell<Node<&'a T>>>>,
}

/// A node represents the data and the next [Node] to point in the next.
#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    back: Weak<RefCell<Node<T>>>,
}

impl<'a, T: PartialEq> LinkedList<'a, T> {
    pub fn new() -> Self {
        Self {
            cursor: 0,
            len: 0,
            head: None,
            tail: None,
        }
    }
    pub fn add_to_head(&mut self, data: &'a T) {
        let node = Rc::new(RefCell::new(Node {
            data,
            next: None,
            back: Weak::new(),
        }));
        if self.head.is_none() {
            self.head = Some(node.clone());
            self.tail = Some(node.clone());
        } else {
            if let Some(head) = self.head.as_mut() {
                head.borrow_mut().back = Rc::downgrade(&node.clone())
            }

            node.borrow_mut().next = self.head.as_ref().map(|node| node.clone());

            self.head = Some(node);
        }

        self.len += 1;
    }
    pub fn add_to_tail(&mut self, data: &'a T) {
        let node = Rc::new(RefCell::new(Node {
            data,
            next: None,
            back: Weak::new(),
        }));
        if let Some(curr_node) = self.tail.as_mut() {
            node.borrow_mut().back = Rc::downgrade(&curr_node.clone());
            curr_node.borrow_mut().next = Some(node.clone());
            self.tail = Some(node.clone());
        } else {
            self.tail = Some(node.clone());
            self.head = Some(node.clone());
        }

        self.len += 1;
    }

    pub fn iter(&self) -> LinkedListIterator<'a, T> {
        if let Some(head) = &self.head {
            LinkedListIterator::new(Rc::downgrade(head))
        } else {
            LinkedListIterator::new(Weak::new())
        }
    }

    pub fn clear(&mut self) {
        self.head = None;
        self.tail = None;
        self.len = 0
    }

    pub fn pop_front(&mut self) -> Option<&T> {
        if self.len == 1 {
            let data = self.head.take().map(|node| node.borrow().data);
            self.clear();
            data
        } else {
            self.head.take().map(|node| {
                let node = node.borrow();
                self.head = node.next.as_ref().map(|node| node.clone());

                self.len -= 1;
                node.data
            })
        }
    }

    pub fn pop_back(&mut self) -> Option<&T> {
        self.tail.take().map(|node| {
            let back = node.borrow().back.upgrade();
            self.tail = back;

            self.len -= 1;
            node.borrow().data
        })
    }

    pub fn delete_by_value(&mut self, value: T) -> Option<&T> {
        self.delete_by_predicate(|node| *node.data == value)
    }

    fn delete_by_pos(&mut self, pos: usize) -> Option<&T> {
        if self.len < pos {
            return None;
        }
        let mut count = 0;
        self.delete_by_predicate(|_| {
            count += 1;
            count == pos
        })
    }

    pub fn delete_by_predicate<F>(&mut self, mut predicate: F) -> Option<&T>
    where
        F: FnMut(&Node<&'a T>) -> bool,
    {
        let mut node = self.head.as_ref().map(|node| node.clone());

        while let Some(head) = node.as_ref() {
            if predicate(&*head.as_ref().borrow()) {
                let head = head.borrow();
                return head
                    .back
                    .upgrade()
                    .map(|node| {
                        node.borrow_mut().next = head.next.as_ref().map(|node| node.clone());
                        self.len -= 1;
                        head.data
                    })
                    .or_else(|| self.pop_front());
            } else {
                node = node.and_then(|node| node.borrow().next.as_ref().map(|node| node.clone()));
            }
        }

        None
    }
}

struct LinkedListIterator<'a, T> {
    data: Weak<RefCell<Node<&'a T>>>,
}

impl<'a, T> LinkedListIterator<'a, T> {
    pub fn new(data: Weak<RefCell<Node<&'a T>>>) -> Self {
        Self { data }
    }
}
impl<'a, T> Iterator for LinkedListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.upgrade().and_then(|node| {
            node.borrow().next.as_ref().map(|inner_node| {
                self.data = Rc::downgrade(inner_node);
                node.borrow().data
            })
        })
    }
}

#[cfg(test)]
mod test {

    use super::LinkedList;

    #[test]
    fn should_compute_the_len_of_the_list() {
        let mut list = LinkedList::<f32>::new();
        list.add_to_tail(&2.54);
        list.add_to_head(&54.654);
        list.add_to_tail(&543.42);

        assert_eq!(list.len, 3);
    }
    #[test]
    fn should_add_to_the_head_and_tail_and_linked_correctly() {
        let mut list = LinkedList::<f32>::new();
        list.add_to_tail(&1.00);
        list.add_to_head(&2.54);

        assert_ne!(list.head.unwrap().as_ptr(), list.tail.unwrap().as_ptr());
    }
    #[test]
    fn should_iterate_the_list() {
        let values = [2.54f32, 54.654, 543.42];
        let mut list = LinkedList::<f32>::new();
        list.add_to_head(&543.42);
        list.add_to_head(&54.654);
        list.add_to_head(&2.54);

        for (idx, node) in list.iter().enumerate() {
            assert_eq!(values[idx], *node);
        }
    }

    #[test]
    fn should_delete_a_node_by_its_value() {
        let mut list = LinkedList::<f32>::new();
        list.add_to_head(&543.42);
        list.add_to_head(&54.654);
        list.add_to_head(&2.54);

        let first = list.delete_by_value(54.654);

        assert_eq!(Some(&54.654), first);
        assert_eq!(list.len, 2);

        let mut list2 = list;
        let second = list2.delete_by_value(2.54);

        assert_eq!(Some(&2.54), second);
        assert_eq!(list2.len, 1);
    }

    #[test]
    fn should_delete_by_pos() {
        let mut list = LinkedList::<f32>::new();
        list.add_to_head(&543.42);
        list.add_to_head(&54.654);
        list.add_to_head(&2.54);

        let first = list.delete_by_pos(1);

        assert_eq!(Some(&54.654), first);
        assert_eq!(list.len, 2);

        let mut list2 = list;
        let second = list2.delete_by_pos(5415);

        assert_eq!(second, None);
        assert_eq!(list2.len, 2);
    }

    #[test]
    fn shoud_pop_back() {
        let mut list = LinkedList::<f32>::new();
        list.add_to_head(&543.42);
        list.add_to_head(&2.42);

        assert_eq!(Some(&543.42), list.pop_back());
        list.pop_back();
        assert_eq!(None, list.pop_back())
    }

    #[test]
    fn shoud_pop_front() {
        let mut list = LinkedList::<f32>::new();
        list.add_to_head(&543.42);
        list.add_to_head(&2.42);

        assert_eq!(Some(&2.42), list.pop_front());
        list.pop_front();
        assert_eq!(None, list.pop_front())
    }
}
