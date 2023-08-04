struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}


impl<T> List<T> {
    pub fn new() -> Self {
        // self.head = Link<T>::new();

        List { head: None }
    }

    pub fn get_last(&self) -> Option<&T> {
        let mut current = &self.head;

        while let Some(node) = current {
            if node.next.is_none() {
                return Some(&node.elem);
            }
            current = &node.next;
        }

        None
    }

    pub fn push(&mut self, elem: T) {
        let n_node = Node {
            elem,
            next: self.head.take(),
        };

        self.head = Some(Box::new(n_node));
    }

    pub fn push_on_tail(&mut self, elem: T) {
        let n_node = Node {
            elem,
            next: None,
        };

        if self.head.is_none() {
            self.head = Some(Box::new(n_node));
        } else {
            let mut current = &mut self.head;
            while let Some(ref mut node) = current {
                if node.next.is_none() {
                    node.next = Some(Box::new(n_node));
                    break;
                } else {
                    current = &mut node.next;
                }
            }
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|n| {
            self.head = n.next;
            n.elem
        })
    }


    pub fn pop_on_tail(&mut self) -> Option<T> {
        if (self.head.is_none()) {
            return None;
        }

        if (self.head.as_ref().unwrap().next.is_none()) {
            return self.head.take().map(|n| n.elem);
        }

        //let mut current  = &mut self.head.unwrap();
        let mut current = self.head.as_mut().unwrap();

        // while let Some(node) = current {
        //     if node.next.is_none() {
        //         return (node.as_mut().unwrap().elem);
        //     } else {
        //         // current = node.next;
        //     }
        // }

        while !current.next.is_none() {
            if (current.next.as_mut().unwrap().next.is_none()) {
                break;
            }

            current = current.next.as_mut().unwrap();
        }

        current.next.take().map(|n| n.elem)
    }
}


impl<T> Drop for List<T> {
    fn drop(&mut self) {

    }
}



#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}


fn main() {
    println!("Hello, world!");
}
