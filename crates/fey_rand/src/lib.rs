//! Random number generation.

use fey_math::Float;
use rand::Rng;
use rand::distr::uniform::{SampleRange, SampleUniform};
use rand::distr::{Distribution, StandardUniform};
use rand::seq::{IndexedMutRandom, IndexedRandom};
use rand_core::impls::fill_bytes_via_next;
use rand_core::{RngCore, SeedableRng};
use serde::{Deserialize, Serialize};

/// A simple, fast, seedable random number generator.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[repr(transparent)]
pub struct Rand(pub u64);

const PHI: u64 = 0x9e3779b97f4a7c15;

impl Rand {
    /// Create a new RNG with a random seed.
    #[inline]
    pub fn new() -> Self {
        Self(rand::random())
    }

    /// Create a new RNG from a specific seed.
    #[inline]
    pub fn from_seed(seed: u64) -> Self {
        Self(seed)
    }

    /// Get the RNG's current seed, which can be used to
    /// reset the RNG to a specific, deterministic state.
    #[inline]
    pub fn seed(&self) -> u64 {
        self.0
    }

    /// Set the RNG's current seed.
    #[inline]
    pub fn set_seed(&mut self, seed: u64) {
        self.0 = seed;
    }

    /// Has the probility of `chance` to return true.
    #[inline]
    pub fn chance<F: Float>(&mut self, chance: F) -> bool {
        Rng::random_bool(self, chance.to_f64())
    }

    /// Generate a random boolean.
    #[inline]
    pub fn boolean(&mut self) -> bool {
        self.chance(0.5f32)
    }

    /// Generate a new random value.
    #[inline]
    pub fn random<T>(&mut self) -> T
    where
        StandardUniform: Distribution<T>,
    {
        Rng::random(self)
    }

    /// Generate a random value in the provided range.
    #[inline]
    pub fn range<T, R>(&mut self, range: R) -> T
    where
        T: SampleUniform,
        R: SampleRange<T>,
    {
        Rng::random_range(self, range)
    }

    /// Generate a value using the specified distribution.
    #[inline]
    pub fn sample<T, D: Distribution<T>>(&mut self, distr: D) -> T {
        Rng::sample(self, distr)
    }

    /// Has the probility of `numerator / denominator` to return true.
    #[inline]
    pub fn ratio(&mut self, numerator: u32, denominator: u32) -> bool {
        Rng::random_ratio(self, numerator, denominator)
    }

    /// Randomly shuffle the elements in the slice.
    #[inline]
    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        let mut n = slice.len();
        while n > 1 {
            let k = self.range(0..n);
            n -= 1;
            slice.swap(n, k);
        }
    }

    /// Choose a random element from the slice. Panics if the slice is empty.
    #[inline]
    pub fn choose<'a, T>(&mut self, slice: &'a [T]) -> Option<&'a T> {
        slice.choose(self)
    }

    /// Choose a random element from the slice. Panics if the slice is empty.
    #[inline]
    pub fn choose_mut<'a, T>(&mut self, slice: &'a mut [T]) -> Option<&'a mut T> {
        slice.choose_mut(self)
    }
}

impl RngCore for Rand {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.0 = self.0.wrapping_add(PHI);
        let mut x = self.0;
        x = (x ^ (x >> 33)).wrapping_mul(0x62A9D9ED799705F5);
        x = (x ^ (x >> 28)).wrapping_mul(0xCB24D0A5C88C35B3);
        (x >> 32) as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(PHI);
        let mut x = self.0;
        x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
        x ^ (x >> 31)
    }

    #[inline]
    fn fill_bytes(&mut self, dst: &mut [u8]) {
        fill_bytes_via_next(self, dst);
    }
}

impl SeedableRng for Rand {
    type Seed = [u8; 8];

    #[inline]
    fn from_seed(seed: Self::Seed) -> Self {
        Self(u64::from_le_bytes(seed))
    }

    #[inline]
    fn seed_from_u64(state: u64) -> Self {
        Self(state)
    }
}
