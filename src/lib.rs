struct List<T> {
    head: Link<T>,
    size: usize,
}

struct Node<T> {
    item: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

impl<T> List<T> {
    fn new() -> Self {
        List {
            head: None,
            size: 0,
        }
    }

    fn len(&self) -> usize {
        self.size
    }

    fn push(&mut self, item: T) {
        let mut node = Box::new(Node { item, next: None });
        if let Some(head) = self.head.take() {
            node.next = Some(head);
        }
        self.head = Some(node);
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if let Some(head) = self.head.take() {
            self.head = head.next;
            self.size -= 1;
            Some(head.item)
        } else {
            None
        }
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.item)
    }

    fn last(&self) -> Option<&T> {
        self.head.as_ref().map(|mut node| {
            while let Some(next) = node.next.as_ref() {
                node = next;
            }
            &node.item
        })
    }

    fn append(&mut self, item: T) {
        let node = Box::new(Node { item, next: None });
        match &mut self.head {
            None => self.head = Some(node),
            Some(head) => {
                let mut p = head;
                while p.next.is_some() {
                    p = p.next.as_mut().unwrap();
                }
                p.next = Some(node);
            }
        }
        self.size += 1;
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(head) = self.head.take() {
            self.head = head.next;
        }
    }
}

struct IntoIter<T> {
    ptr: Link<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.ptr.take()?;
        let item = node.item;
        self.ptr = node.next;
        Some(item)
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(mut self) -> Self::IntoIter {
        IntoIter {
            ptr: self.head.take(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_pop() {
        let mut list = List::new();

        list.push("foo");
        list.push("bar");
        assert_eq!(list.len(), 2);

        assert_eq!(list.pop(), Some("bar"));
        assert_eq!(list.pop(), Some("foo"));
        assert_eq!(list.pop(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);

        list.push(42);
        assert_eq!(list.peek(), Some(&42));
    }

    #[test]
    fn append() {
        let mut list = List::new();

        list.append("foo");
        list.append("bar");
        list.append("baz");
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop(), Some("foo"));
        assert_eq!(list.pop(), Some("bar"));
        assert_eq!(list.pop(), Some("baz"));
    }

    #[test]
    fn last() {
        let mut list = List::new();
        assert_eq!(list.last(), None);
        list.push("foo");
        list.push("bar");
        assert_eq!(list.last(), Some(&"foo"));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push("baz");
        list.push("bar");
        list.push("foo");

        let items: Vec<&str> = list.into_iter().collect();
        assert_eq![items, vec!["foo", "bar", "baz"]];
    }

    #[test]
    fn big_drop() {
        let mut list = List::new();
        for i in 1..100_000 {
            list.push(i);
        }
    }
}
