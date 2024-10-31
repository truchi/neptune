use crate::Float;
use std::collections::VecDeque;

// ────────────────────────────────────────────────────────────────────────────────────────────── //

/// A monotonic min deque.
#[derive(Default)]
pub struct MinDeque<T: PartialOrd = Float>(VecDeque<T>);

impl<T: PartialOrd> std::fmt::Debug for MinDeque<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({} elems)", self.0.len()))
    }
}

impl<T: PartialOrd> MinDeque<T> {
    /// Returns the minimum value.
    pub fn min(&self) -> Option<&T> {
        self.0.front()
    }

    /// Removes `old` then pushes `new`.
    pub fn update(&mut self, old: Option<T>, new: T) {
        if let Some(old) = old {
            self.remove(old);
        }

        self.push(new);
    }

    /// Pushes `value`.
    fn push(&mut self, value: T) {
        while let Some(back) = self.0.back() {
            if *back > value {
                self.0.pop_back();
            } else {
                break;
            }
        }

        self.0.push_back(value);
    }

    /// Removes `value`.
    fn remove(&mut self, value: T) {
        if let Some(front) = self.0.front() {
            if value == *front {
                self.0.pop_front();
            }
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────────────────────── //

/// A monotonic max deque.
#[derive(Default)]
pub struct MaxDeque<T: PartialOrd = Float>(VecDeque<T>);

impl<T: PartialOrd> std::fmt::Debug for MaxDeque<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({} elems)", self.0.len()))
    }
}

impl<T: PartialOrd> MaxDeque<T> {
    /// Returns the maximum value.
    pub fn max(&self) -> Option<&T> {
        self.0.front()
    }

    /// Removes `old` then pushes `new`.
    pub fn update(&mut self, old: Option<T>, new: T) {
        if let Some(old) = old {
            self.remove(old);
        }

        self.push(new);
    }

    /// Pushes `value`.
    fn push(&mut self, value: T) {
        while let Some(back) = self.0.back() {
            if *back < value {
                self.0.pop_back();
            } else {
                break;
            }
        }

        self.0.push_back(value);
    }

    /// Removes `value`.
    fn remove(&mut self, value: T) {
        if let Some(front) = self.0.front() {
            if value == *front {
                self.0.pop_front();
            }
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────────────────────── //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::window::Window;

    #[test]
    fn test() {
        for items in [
            [0, 0, 1, 1, 2, 2, 3, 4, 5, 4, 3, 3, 2, 2, 1, 1],
            std::array::from_fn::<i16, 16, _>(|_| rand::random()),
        ] {
            for max_len in 1..=items.len() {
                let mut window = Window::new(max_len);
                let mut max_deq = MaxDeque::default();
                let mut min_deq = MinDeque::default();

                assert!(max_deq.max() == None);
                assert!(min_deq.min() == None);

                for item in items {
                    let old = window.push(item);
                    max_deq.update(old, item);
                    min_deq.update(old, item);

                    assert!(max_deq.max() == window.iter().max());
                    assert!(min_deq.min() == window.iter().min());
                }
            }
        }
    }
}
