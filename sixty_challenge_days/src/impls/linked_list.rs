use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct LinkedList<'a, T> {
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

            node.borrow_mut().next = self.head.clone();

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
            LinkedListIterator::new(Some(head.clone()))
        } else {
            LinkedListIterator::new(None)
        }
    }

    pub fn delete_by_value(&mut self, value: T) -> Option<T> {
        let mut tim = self.head.as_ref().map(|node| node.clone());
        while let Some(head) = tim.as_ref() {
            let now = head.clone();
            let now = now.borrow();
            if *now.data == value {
                if let Some(next) = &head.borrow().next {
                    if let Some(rc) = now.back.upgrade() {
                        rc.borrow_mut().next = Some(next.clone());
                        Some(rc.borrow().data)
                    } else {
                        None
                    }
                    .or_else(|| {
                        self.head = Some(next.clone());
                        None
                    });
                } else {
                    if let Some(rc) = now.back.upgrade() {
                        rc.borrow_mut().next = None;
                    }
                }

                tim = None
            } else {
                tim = tim.and_then(|node| node.borrow().next.clone());
            }
        }

        None
    }
}

// impl<'a, T> IntoIterator for LinkedList<'a, T> {
//     type Item = &'a T;

//     type IntoIter = LinkedListIterator<'a, T>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.iter()
//     }
// }

struct LinkedListIterator<'a, T> {
    data: Weak<RefCell<Node<&'a T>>>,
}

impl<'a, T> LinkedListIterator<'a, T> {
    pub fn new(data: Option<Rc<RefCell<Node<&'a T>>>>) -> Self {
        Self {
            data: data.map(|d| Rc::downgrade(&d)).unwrap_or(Weak::new()),
        }
    }
}
impl<'a, T> Iterator for LinkedListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.upgrade().map(|node| {
            if let Some(node_rc) = node.borrow().next.as_ref() {
                self.data = Rc::downgrade(node_rc);
            }

            node.borrow().data
        })
    }
}

#[cfg(test)]
mod test {

    use super::{LinkedList, Node};
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn should_compute_the_len_of_the_list() {
        let mut list = LinkedList::<f32>::new();
        list.add_to_tail(&2.54);
        list.add_to_head(&54.654);
        list.add_to_tail(&543.42);

        assert_eq!(list.len, 3);
    }
    // #[test]
    // fn should_add_to_the_head_and_tail_and_linked_correctly() {
    //     let mut list = LinkedList::<f32>::new();
    //     list.add_to_tail(&1.00);
    //     list.add_to_head(&2.54);

    //     let back = Rc::new(RefCell::new(Node {
    //         data: &1.00,
    //         next: None,
    //         back: None,
    //     }));
    //     let front = Rc::new(RefCell::new(Node {
    //         data: &2.54,
    //         next: Some(back.clone()),
    //         back: Some(Rc::downgrade(&back.clone())),
    //     }));

    //     assert_eq!(list.tail, Some(back));
    //     assert_eq!(list.head, Some(front));
    // }
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

        list.delete_by_value(543.42);

        for node in list.iter() {
            assert_ne!(*node, 543.42);
        }

        list.delete_by_value(2.54);

        for node in list.iter() {
            assert_ne!(*node, 2.54);
        }
    }
}
