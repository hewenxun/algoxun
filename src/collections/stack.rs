use std::slice;

pub struct Stack<T>(Vec<T>);

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn push(&mut self, item: T) {
        self.0.push(item)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    pub fn top(&self) -> Option<&T> {
        self.0.last()
    }

    pub fn top_mut(&mut self) -> Option<&mut T> {
        self.0.last_mut()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> slice::Iter<T> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<T> {
        self.0.iter_mut()
    }
}

impl<T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Stack<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Stack<T> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integers_stack() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.push(4);
        stack.push(5);
        stack.push(6);
        assert_eq!(stack.is_empty(), false);
        assert_eq!(stack.size(), 6);
        assert_eq!(stack.top(), Some(&6));
        *stack.top_mut().unwrap() = 0;
        assert_eq!(stack.top(), Some(&0));
        stack.pop();
        for num in stack {
            print!("{:?}", num);
        }
        println!();
    }

    #[test]
    fn chars_stack() {
        let mut stack = Stack::with_capacity(10);
        stack.push('a');
        stack.push('b');
        stack.push('c');

        for c in &stack {
            print!("{:?}", *c);
        }
        println!();
    }

    #[test]
    fn strs_stack() {
        let mut stack = Stack::with_capacity(10);
        stack.push("a");
        stack.push("b");
        stack.push("c");
        for &str in &stack {
            print!("{:?}", str);
        }
        println!();
    }

    #[test]
    fn strings_stack() {
        let mut stack = Stack::with_capacity(10);
        stack.push("a".to_string());
        stack.push("b".to_string());
        stack.push("c".to_string());
        for string in &mut stack {
            *string = "e".to_string();
            print!("{:?}", string);
        }
    }
}