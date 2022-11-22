use std::fmt::{self, Display};
use std::option::Option;

type NodeNext<T> = Option<Box<Node<T>>>;

pub struct LinkedList<T> {
    head: NodeNext<T>,
    size: usize,
}

struct Node<T> {
    value: T,
    next: NodeNext<T>,
}

impl<T> Node<T> {
    pub fn new(value: T, next: Option<Box<Node<T>>>) -> Node<T> {
        Node {
            value: value,
            next: next,
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            size: 0,
        }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.get_size() == 0
    }

    pub fn push_front(&mut self, value: T) {
        let new_node: Box<Node<T>> = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node);
        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let node: Box<Node<T>> = self.head.take()?;
        self.head = node.next;
        self.size -= 1;
        Some(node.value)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { next: &self.head }
    }
}

impl<T: Display> fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current: &NodeNext<T> = &self.head;
        let mut result = String::new();
        loop {
            match current {
                Some(node) => {
                    result = format!("{} {}", result, node.value);
                    current = &node.next;
                }
                None => break,
            }
        }
        write!(f, "{}", result)
    }
}

pub struct IntoIter<T>(NodeNext<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.0.take();
        match next {
            None => None,
            Some(next_node) => {
                let value = next_node.value;
                self.0 = next_node.next;
                Some(value)
            }
        }
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(mut self) -> Self::IntoIter {
        IntoIter(self.head.take())
    }
}

pub struct Iter<'a, T> {
    next: &'a NodeNext<T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match &self.next {
            None => None,
            Some(next) => {
                self.next = &next.next;
                Some(&next.value)
            }
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node) = current {
            current = node.next.take();
        }
    }
}
