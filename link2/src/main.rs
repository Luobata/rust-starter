#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            next: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList {
            head: None,
        }
    }

    fn push(&mut self, value: T) {
        let new_node = Box::new(Node::new(value));
        match self.head.as_mut() {
            Some(node) => {
                node.next = Some(new_node);
            }
            None => {
                self.head = Some(new_node);
            }
        }
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);

    println!("{:?}", list);
}
