pub struct LinkedList<T> {
    head: Link<T>,
}

impl LinkedList<u32> {}
impl LinkedList<String> {}

impl<T> LinkedList<T> {
    fn empty() -> Self {
        Self { head: None }
    }

    fn push(&mut self, element: T) {
        let old_head = self.head.take();
        let new_head = Box::new(Node {
            element,
            next: old_head,
        });

        self.head = Some(new_head);
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|n| {
            self.head = n.next;
            n.element
        })
    }

    fn peak(&mut self) -> Option<&T> {
        self.head.as_ref().map(|n| &n.element)
    }
}

struct Node<T> {
    element: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut list = LinkedList::empty();
        list.push(1024);
        list.push(42);
    }
}
