use std::fmt;
use std::option::Option;
use std::thread::current;

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T, next: Option<Box<Node<T>>>) -> Node<T> {
        Node {value: value, next: next}
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {head: None, size: 0}
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

    pub fn push_back(&mut self, value: T) {
        let new_node: Box<Node<T>> = Box::new(Node::new(value, None));

        let mut current_link = &mut self.head;
        while let Some(ref mut node) = current_link {
            current_link = &mut node.next;
        }
        *current_link = Some(new_node);
        self.size += 1;
    }
    
    pub fn pop_front(&mut self) -> Option<T> {
        let node: Box<Node<T>> = self.head.take()?;
        self.head = node.next;
        self.size -= 1;
        Some(node.value)
    }
}


impl<T: fmt::Display> fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current: &Option<Box<Node<T>>> = &self.head;
        let mut result = String::new();
        loop {
            match current {
                Some(node) => {
                    result = format!("{} {}", result, node.value);
                    current = &node.next;
                },
                None => break,
            }
        }
        write!(f, "{}", result)
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

impl<T: Clone> Clone for LinkedList<T> {
    fn clone(&self) -> LinkedList<T> {
        let mut res = LinkedList::<T>::new();
        let mut current = &self.head;
        while let Some(node) = current {
            res.push_back(node.value.clone());
            current = &node.next;   
        }
        res
    }
}

impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other:&LinkedList<T>) -> bool {
        if self.get_size() != other.get_size() {
            return false
        }
        let mut current_self = &self.head;
        let mut current_other = &other.head;

        loop {
            match(current_self, current_other) {
                (Some(node_self), Some(node_other)) => {
                    if node_self.value != node_other.value {
                        return false;
                    }
                    current_self = &node_self.next;
                    current_other = &node_other.next;
                }
                (None, None) => {
                    return true;
                }
                _ => {
                    return false;
                }
            }
        }
    }
}

// impl<T> ComputeNorm for LinkedList<T> {
// }

pub struct LinkedListIter<'a, T> {
    current: &'a Option<Box<Node<T>>>,
}

impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;
    
    fn next(&mut self) -> Option<&'a T> {
        match self.current {
            Some(node) => {
                self.current = &node.next;
                Some(&node.value)
            }
            None => None,
        }
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;
    type IntoIter = LinkedListIter<'a, T>;
    
    fn into_iter(self) -> LinkedListIter<'a, T> {
        LinkedListIter {
            current: &self.head,
        }
    }
}


