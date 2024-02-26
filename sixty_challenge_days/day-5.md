# Day 5: Beyond the basic of Linked List - 25-02-2024

## My Answers

### Focus Areas
#### Iterator Refinement
I tried to refactor the ``next`` implementation to allow to iterate without consuming the list, but the compiler accused an error: "mismatched types expected enum `Option<Box<Node<T>>>` found mutable reference `&mut Option<Box<Node<T>>>`"
```rs
impl<'a, T: 'a> Iterator for &'a LinkedList<T> {
    type Item = &'a T;

   fn next(&mut self) -> Option<&'a T> {
    self.head.as_mut().map(|node| {
        self.head = &mut node.next;
        &node.data
    })
}

}
```
I don't know how can I resolve this 😭.

### Add to Tail... Or Not?
I implemented the ``add_to_tail`` to our Linked List and compiles, but it is not right because the ``head`` and ``tail`` is not linking. For example, if the list has only one value, the head tail should have the same reference, but it's not possible in Rust to two variables owns the same value. I tried to change its types for ``Rc<Option<Box>>`` to allow both to have the same reference, but it is not worked.

```rs
pub fn add_to_tail(&mut self, data: T) {
    let node = Box::new(Node { data, next: None });
    if let Some(curr_node) = self.tail.as_mut() {
        curr_node.next = Some(node);
        self.tail = curr_node.next.take();
    } else {
        self.tail = Some(node);
    }
}
```

### When to Choose Linked Lists
The ``Vec<T>`` is great for data manipulate since it can grow and shrink as long is necessary. It has three fields: a pointer to the data, capacity and length. The capacity is important because it is the information that tells the amount of memory allocated for the Vector. If the data pass the limit, the Rust has to resize its size and move the entire data to a new location with the capacity to saving the Vector data.

## Answer with help
```rs
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct LinkedList<T> {
    len: usize,
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

/// A node represents the data and the next [Node] to point in the next.
#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            len: 0,
            head: None,
            tail: None,
        }
    }
    pub fn add_to_head(&mut self, data: T) {
        let node = Rc::new(RefCell::new(Node { data, next: None }));
        if self.head.is_none() {
            self.head = Some(node.clone());
            self.tail = Some(node.clone());
        } else {
            node.borrow_mut().next = self.head.clone();
            self.head = Some(node);
        }

        self.len += 1;
    }
    pub fn add_to_tail(&mut self, data: T) {
        let node = Rc::new(RefCell::new(Node { data, next: None }));
        if let Some(curr_node) = self.tail.as_mut() {
            curr_node.borrow_mut().next = Some(node.clone());
            self.tail = Some(node.clone());
        } else {
            self.tail = Some(node.clone());
            self.head = Some(node.clone());
        }

        self.len += 1;
    }
}
```