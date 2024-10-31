use crate::{stats::Stats, window::Window, Float, K1, K2, K3, K4, K5, K6, K7, K8, KS};

/// Values and stats for a symbol.
#[derive(Debug)]
pub struct Symbol {
    values: Window,
    stats: [Stats; 8],
}

impl Default for Symbol {
    fn default() -> Self {
        Self {
            values: Window::new(K8),
            stats: Default::default(),
        }
    }
}

impl Symbol {
    /// Returns `[min, max, last, average, variance]` for `k`.
    pub fn stats(&self, k: usize) -> Option<[Float; 5]> {
        debug_assert!(KS.contains(&k));

        if self.values.is_empty() {
            return None;
        }

        let stats = &self.stats[k - 1];
        let (min, max) = stats.min_max().unwrap_or_default();
        let (average, variance) = stats.average_variance(self.values.len());
        let last = self.values.get(0).copied().unwrap_or_default();

        Some([min, max, last, average, variance])
    }

    /// Adds `values` for that symbol.
    pub fn add_batch(&mut self, values: &[Float]) {
        for &value in values {
            self.stats[7].update(self.values.push(value), value);
            self.stats[6].update(self.values.get(K7).copied(), value);
            self.stats[5].update(self.values.get(K6).copied(), value);
            self.stats[4].update(self.values.get(K5).copied(), value);
            self.stats[3].update(self.values.get(K4).copied(), value);
            self.stats[2].update(self.values.get(K3).copied(), value);
            self.stats[1].update(self.values.get(K2).copied(), value);
            self.stats[0].update(self.values.get(K1).copied(), value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let items = std::array::from_fn::<_, K4, _>(|_| rand::random::<i16>() as Float);
        let mut symbol = Symbol::default();

        for k in KS {
            assert!(symbol.stats(k).is_none());
        }

        for item in items {
            symbol.add_batch(&[item]);

            symbol.stats[0].assert(symbol.values.iter().copied().take(K1));
            symbol.stats[1].assert(symbol.values.iter().copied().take(K2));
            symbol.stats[2].assert(symbol.values.iter().copied().take(K3));
            symbol.stats[3].assert(symbol.values.iter().copied().take(K4));
        }
    }
}
