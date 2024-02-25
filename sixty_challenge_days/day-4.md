# Day-4: Build a (Basic) Linked List - 25-02-2024

## Exercises

### Recap: Review
#### The mental model of nodes, data, and pointers.
The nodes represent a *Node* that holds a data of the list and a pointer (reference) to the next ``Node`` in the list. With that logic, the List can be Linked between ``Nodes``.
#### Why Linked List help us overcome limitations of arrays or fixed structs.
As the Linked List is a set of ``Nodes`` and each node holds a reference to the next, the list can be grown and shrink as new node in added in the List. Each node ca be separate spaces in the memory, so it is not overhead the memory.

### Node Implementation:
#### Discussion
We have to use ``Option`` enum here because a node may not have a pointer to the next one. For example, a list with only one node does not have the next one. So the ``next`` has to be ``None``.
The smart pointer ``Box`` is necessary to allow the Rust known the size of the data recursively. Without it, the compiler could not estimate the amount of memory allocate to the next node. The box allow us to save the data on the Heap, so it has size type and compiler knows how allocate memory for its.


#### Implementation

```rs
use std::mem::swap;

#[derive(Debug)]
struct LinkedList<T> {
    /// The first ``Node<T>`` of the list to keep tracking
    head: Option<Box<Node<T>>>,
}

/// A node represents the data and the next [Node] to point in the List.
#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    /// Create a new intance of [LinkedList]
    pub fn new() -> Self {
        Self { head: None }
    }

    /// Add a value [T] to the front of the List
    /// This have O(1) performance
    pub fn add_to_head(&mut self, data: T) {
        let mut node = Box::new(Node { data, next: None });
        if self.head.is_none() {
            self.head = Some(node);
        } else {
            swap(&mut node.next, &mut self.head);
            self.head = Some(node);
        }
    }
}

impl<T> Iterator for LinkedList<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }
}

#[cfg(test)]
mod test {
    use super::LinkedList;

    #[test]
    fn should_add_to_the_head_of_list() {
        let mut list = LinkedList::<f32>::new();
        list.add_to_head(2.54);
        list.add_to_head(54.654);
        list.add_to_head(543.42);

        assert_eq!(list.count(), 3);
    }
    #[test]
    fn should_iterate_the_list() {
        let values = [2.54f32, 54.654, 543.42];
        let mut list = LinkedList::<f32>::new();
        list.add_to_head(543.42);
        list.add_to_head(54.654);
        list.add_to_head(2.54);

        for (idx, node) in list.enumerate() {
            assert_eq!(values[idx], node);
        }
    }
}
```

### My questions
1. I have to use swap to update the ``head`` field. Rust did not allow me to mutate the head been a mutable reference. The error was *cannot move out of `self.head` which is behind a mutable reference* when I tried to ``self.node = self.head``.