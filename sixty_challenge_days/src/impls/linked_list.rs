use std::mem::swap;

#[derive(Debug)]
struct LinkedList<T> {
    /// The first ``Node<T>`` of the list to keep tracking
    head: Option<Box<Node<T>>>,
}

/// A node represents the data and the next [Node] to point in the next.
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
