use crate::Float;
use std::collections::VecDeque;

/// A sliding window with a length limit.
pub struct Window<T = Float> {
    max_len: usize,
    items: VecDeque<T>,
}

impl<T> std::fmt::Debug for Window<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({} elems)", self.len()))
    }
}

impl<T> Window<T> {
    /// Creates a new `Window` with (non-zero) `max_len`.
    pub fn new(max_len: usize) -> Self {
        assert!(max_len > 0, "Max len must be > 0");

        Self {
            max_len,
            items: VecDeque::with_capacity(max_len),
        }
    }

    /// Returns the length of the `Window`.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns `true` is the `Window` is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the item at `index` (in reverse chronological order).
    pub fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }

    /// Returns an iterator over items (in reverse chronological order).
    #[cfg(test)]
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }

    /// Adds `item` to the window, returning the old item at max length.
    pub fn push(&mut self, item: T) -> Option<T> {
        let old = (self.len() == self.max_len)
            .then(|| self.items.pop_back())
            .flatten();

        self.items.push_front(item);
        debug_assert!(self.len() <= self.max_len);

        old
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut window = Window::new(3);

        assert!(window.len() == 0);
        assert!(window.push(0) == None);
        assert!(window.len() == 1);
        assert!(window.push(1) == None);
        assert!(window.len() == 2);
        assert!(window.push(2) == None);
        assert!(window.len() == 3);
        assert!(window.push(3) == Some(0));
        assert!(window.len() == 3);
        assert!(window.push(4) == Some(1));
        assert!(window.len() == 3);
        assert!(window.push(5) == Some(2));
        assert!(window.len() == 3);
        assert!(window.push(6) == Some(3));
        assert!(window.len() == 3);

        assert!(window.get(0) == Some(&6));
        assert!(window.get(1) == Some(&5));
        assert!(window.get(2) == Some(&4));
        assert!(window.get(3) == None);

        assert!(window.iter().collect::<Vec<_>>() == [&6, &5, &4]);
    }
}
