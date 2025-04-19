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
    pub fn new() -> List<T> {
        List { head: None }
    }

    pub fn push(&mut self, value: T) {
        let new_node = Node { value, next: None };

        match self.head.take() {
            None => {
                // If the list is empty, set the new node as the head
                self.head = Some(new_node);
            }
            Some(old_head) => {
                // Create a new node with the value and the old head as next
                let mut new_head = new_node;
                new_head.next = Some(Box::new(old_head));
                self.head = Some(new_head);
            }
        }
    }

    pub fn pop(&mut self) {
        if let Some(head) = self.head.take() {
            // Set the head to be the next node
            self.head = match head.next {
                Some(next_node) => Some(*next_node),
                None => None,
            };
        }
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current = &self.head;

        while let Some(node) = current {
            count += 1;
            current = match &node.next {
                Some(boxed_node) => &Some(**boxed_node),
                None => &None,
            };
        }

        count
    }
}
