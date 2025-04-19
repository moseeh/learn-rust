/// A singly linked list of generic values.
#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    /// Create a new empty list.
    pub fn new() -> Self {
        List { head: None }
    }

    /// Push a value onto the front of the list.
    pub fn push(&mut self, value: T) {
        let new_node = Node {
            value,
            next: self.head.take().map(Box::new),
        };
        self.head = Some(new_node);
    }

    /// Pop (remove) the first element of the list.
    pub fn pop(&mut self) {
        if let Some(node) = self.head.take() {
            self.head = node.next.map(|boxed| *boxed);
        }
    }

    /// Return the number of elements in the list.
    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            count += 1;
            current = node.next.as_deref();
        }
        count
    }
}
