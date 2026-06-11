//! Random number generation.
//!
//! The C++ implementation relies on `std::minstd_rand`: the algorithm only needs fast,
//! reasonably distributed random numbers, the generator quality is not critical.
//! The same LCG engine is reproduced here. The derived helpers (`uniform_below`, `shuffle`)
//! are distributionally equivalent to `std::uniform_int_distribution` and `std::shuffle`,
//! but not draw-for-draw identical to a specific C++ standard library.

/// Linear congruential generator with the parameters of C++ `std::minstd_rand`
/// (Park-Miller: `x[n+1] = x[n] * 48271 mod (2^31 - 1)`).
pub struct MinstdRand {
    state: u32,
}

impl MinstdRand {
    const MODULUS: u64 = 2_147_483_647; // 2^31 - 1
    const MULTIPLIER: u64 = 48_271;

    /// Seeds the generator like `std::minstd_rand::seed`: a seed mapping to state 0 becomes 1.
    pub fn new(seed: u64) -> Self {
        let state = (seed % Self::MODULUS) as u32;
        Self {
            state: if state == 0 { 1 } else { state },
        }
    }

    /// Returns the next raw value, uniformly distributed in [1, 2^31 - 2].
    #[inline]
    pub fn next(&mut self) -> u32 {
        self.state = ((self.state as u64 * Self::MULTIPLIER) % Self::MODULUS) as u32;
        self.state
    }

    /// Returns a uniform value in [0, n). Rejection sampling avoids modulo bias.
    #[inline]
    pub fn uniform_below(&mut self, n: u32) -> u32 {
        debug_assert!(n >= 1);
        let range = (Self::MODULUS - 1) as u32; // next() yields `range` distinct values
        let limit = range - range % n;
        loop {
            let value = self.next() - 1; // uniform in [0, range)
            if value < limit {
                return value % n;
            }
        }
    }

    /// Fisher-Yates shuffle driven by this generator.
    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        for i in (1..slice.len()).rev() {
            let j = self.uniform_below(i as u32 + 1) as usize;
            slice.swap(i, j);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MinstdRand;

    #[test]
    fn matches_cpp_minstd_rand() {
        // First values of std::minstd_rand seeded with 1.
        let mut rng = MinstdRand::new(1);
        assert_eq!(rng.next(), 48_271);
        assert_eq!(rng.next(), 182_605_794);

        // The C++ standard states that the 10000th consecutive invocation
        // of a default-constructed std::minstd_rand produces 399268537.
        let mut rng = MinstdRand::new(1);
        let mut value = 0;
        for _ in 0..10_000 {
            value = rng.next();
        }
        assert_eq!(value, 399_268_537);
    }

    #[test]
    fn zero_seed_maps_to_one() {
        let mut a = MinstdRand::new(0);
        let mut b = MinstdRand::new(1);
        assert_eq!(a.next(), b.next());
    }

    #[test]
    fn uniform_below_is_in_range() {
        let mut rng = MinstdRand::new(42);
        for n in 1..50 {
            for _ in 0..100 {
                assert!(rng.uniform_below(n) < n);
            }
        }
    }
}
