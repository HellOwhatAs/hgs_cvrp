//! Simple data structure to represent circle sectors.
//!
//! Angles are measured in [0, 65535] instead of [0, 359] so that modulo operations
//! are much faster (since 2^16 = 65536).
//! Credit to Fabian Giesen ("Intervals in modular arithmetic") for implementation
//! tips regarding interval overlaps in modular arithmetics.

#[derive(Clone, Copy, Default)]
pub struct CircleSector {
    pub start: i32,
    pub end: i32,
}

impl CircleSector {
    /// Positive modulo 65536.
    #[inline]
    pub fn positive_mod(i: i32) -> i32 {
        // 1) Using the formula positive_mod(n, x) = (n % x + x) % x
        // 2) "n % 65536" is compiled as "n & 0xffff" for faster calculations
        (i % 65536 + 65536) % 65536
    }

    /// Initializes a circle sector from a single point.
    pub fn initialize(&mut self, point: i32) {
        self.start = point;
        self.end = point;
    }

    /// Tests if a point is enclosed in the circle sector.
    pub fn is_enclosed(&self, point: i32) -> bool {
        Self::positive_mod(point - self.start) <= Self::positive_mod(self.end - self.start)
    }

    /// Tests overlap of two circle sectors.
    pub fn overlap(sector1: &CircleSector, sector2: &CircleSector) -> bool {
        Self::positive_mod(sector2.start - sector1.start)
            <= Self::positive_mod(sector1.end - sector1.start)
            || Self::positive_mod(sector1.start - sector2.start)
                <= Self::positive_mod(sector2.end - sector2.start)
    }

    /// Extends the circle sector to include an additional point.
    /// Done in a "greedy" way, such that the resulting circle sector is the smallest.
    pub fn extend(&mut self, point: i32) {
        if !self.is_enclosed(point) {
            if Self::positive_mod(point - self.end) <= Self::positive_mod(self.start - point) {
                self.end = point;
            } else {
                self.start = point;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CircleSector;

    #[test]
    fn enclosure_and_overlap() {
        let mut sector = CircleSector::default();
        sector.initialize(1000);
        assert!(sector.is_enclosed(1000));
        sector.extend(2000);
        assert!(sector.is_enclosed(1500));
        assert!(!sector.is_enclosed(3000));

        // Sector wrapping around 0.
        let mut wrapping = CircleSector::default();
        wrapping.initialize(65000);
        wrapping.extend(500);
        assert!(wrapping.is_enclosed(0));
        assert!(!wrapping.is_enclosed(30000));

        assert!(CircleSector::overlap(&sector, &sector));
        assert!(!CircleSector::overlap(&sector, &wrapping));
    }
}
