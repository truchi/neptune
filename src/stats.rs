use crate::{
    min_max_deque::{MaxDeque, MinDeque},
    Float,
};

/// Stats for a symbol.
#[derive(Default, Debug)]
pub struct Stats {
    min_deq: MinDeque,
    max_deq: MaxDeque,
    sum_of_values: Float,
    sum_of_squares: Float,
}

impl Stats {
    /// Returns the current `(min, max)`.
    pub fn min_max(&self) -> Option<(Float, Float)> {
        match (self.min_deq.min().copied(), self.max_deq.max().copied()) {
            (None, None) => None,
            (Some(min), Some(max)) => Some((min, max)),
            _ => {
                debug_assert!(false, "unreachable");
                None
            }
        }
    }

    /// Returns the current `(average, variance)`.
    pub fn average_variance(&self, len: usize) -> (Float, Float) {
        let average = self.sum_of_values / len as Float;
        let variance = (self.sum_of_squares / len as Float) - (average * average);

        (average, variance)
    }

    /// Updates this `Stats` with `old` and `new` values.
    pub fn update(&mut self, old: Option<Float>, new: Float) {
        self.min_deq.update(old, new);
        self.max_deq.update(old, new);

        if let Some(old) = old {
            self.sum_of_values -= old;
            self.sum_of_squares -= old * old;
        }

        self.sum_of_values += new;
        self.sum_of_squares += new * new;
    }

    /// Asserts `(min, max, average, variance)` against `items`.
    #[cfg(test)]
    pub fn assert(&self, items: impl Iterator<Item = Float>) {
        let items = items.collect::<Vec<_>>();

        let (expected_min, expected_max, expected_average, expected_variance) = {
            let min = items.iter().copied().reduce(Float::min);
            let max = items.iter().copied().reduce(Float::max);
            let average = items.iter().sum::<Float>() / items.len() as Float;
            let variance = items
                .iter()
                .map(|item| (item - average) * (item - average))
                .sum::<Float>()
                / items.len() as Float;

            (min, max, average, variance)
        };

        fn eq(a: Float, b: Float) -> bool {
            // NOTE: may be flaky...
            const EPSILON: Float = 1e-4;

            (a - b).abs() < EPSILON
        }

        match (self.min_max(), expected_min, expected_max) {
            (None, None, None) => {}
            (Some((min, max)), Some(expected_min), Some(expected_max)) => {
                assert!(eq(min, expected_min));
                assert!(eq(max, expected_max));
            }
            _ => panic!(),
        }

        let (average, variance) = self.average_variance(items.len());
        assert!(eq(average, expected_average));
        assert!(eq(variance, expected_variance));
    }
}

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
            let items = items
                .into_iter()
                .map(|item| item as Float)
                .collect::<Vec<_>>();

            for max_len in 1..=items.len() {
                let mut window = Window::new(max_len);
                let mut stats = Stats::default();

                assert!(stats.min_max() == None);

                for &item in &items {
                    let old = window.push(item);
                    stats.update(old, item);

                    stats.assert(window.iter().copied());
                }
            }
        }
    }
}
