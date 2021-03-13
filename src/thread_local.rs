use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ops::RangeBounds;
use std::thread;

use instant::Instant;

use crate::Rng;

std::thread_local! {
    static RNG: Rng = Rng::with_seed({
        let mut hasher = DefaultHasher::new();
        Instant::now().hash(&mut hasher);
        thread::current().id().hash(&mut hasher);
        let hash = hasher.finish();
        (hash << 1) | 1
    });
}

impl Rng {
    /// Creates a new random number generator seeded from the thread-local one.
    #[inline]
    pub fn new() -> Rng {
        Rng::with_seed(
            RNG.try_with(|rng| rng.u64(..))
                .unwrap_or(0x4d595df4d0f33173),
        )
    }
}

impl Default for Rng {
    #[inline]
    fn default() -> Rng {
        Rng::new()
    }
}

/// Initializes the thread-local generator with the given seed.
#[inline]
pub fn seed(seed: u64) {
    RNG.with(|rng| rng.seed(seed))
}

/// Generates a random `bool`.
#[inline]
pub fn bool() -> bool {
    RNG.with(|rng| rng.bool())
}

/// Generates a random `char` in ranges a-z and A-Z.
#[inline]
pub fn alphabetic() -> char {
    RNG.with(|rng| rng.alphabetic())
}

/// Generates a random `char` in ranges a-z, A-Z and 0-9.
#[inline]
pub fn alphanumeric() -> char {
    RNG.with(|rng| rng.alphanumeric())
}

/// Generates a random `char` in range a-z.
#[inline]
pub fn lowercase() -> char {
    RNG.with(|rng| rng.lowercase())
}

/// Generates a random `char` in range A-Z.
#[inline]
pub fn uppercase() -> char {
    RNG.with(|rng| rng.uppercase())
}

/// Generates a random digit in the given `base`.
///
/// Digits are represented by `char`s in ranges 0-9 and a-z.
///
/// Panics if the base is zero or greater than 36.
#[inline]
pub fn digit(base: u32) -> char {
    RNG.with(|rng| rng.digit(base))
}

/// Shuffles a slice randomly.
#[inline]
pub fn shuffle<T>(slice: &mut [T]) {
    RNG.with(|rng| rng.shuffle(slice))
}

macro_rules! integer {
    ($t:tt, $doc:tt) => {
        #[doc = $doc]
        ///
        /// Panics if the range is empty.
        #[inline]
        pub fn $t(range: impl RangeBounds<$t>) -> $t {
            RNG.with(|rng| rng.$t(range))
        }
    };
}

integer!(u8, "Generates a random `u8` in the given range.");
integer!(i8, "Generates a random `i8` in the given range.");
integer!(u16, "Generates a random `u16` in the given range.");
integer!(i16, "Generates a random `i16` in the given range.");
integer!(u32, "Generates a random `u32` in the given range.");
integer!(i32, "Generates a random `i32` in the given range.");
integer!(u64, "Generates a random `u64` in the given range.");
integer!(i64, "Generates a random `i64` in the given range.");
integer!(u128, "Generates a random `u128` in the given range.");
integer!(i128, "Generates a random `i128` in the given range.");
integer!(usize, "Generates a random `usize` in the given range.");
integer!(isize, "Generates a random `isize` in the given range.");

/// Generates a random `f32` in range `0..1`.
#[inline]
pub fn f32() -> f32 {
    RNG.with(|rng| rng.f32())
}

/// Generates a random `f64` in range `0..1`.
#[inline]
pub fn f64() -> f64 {
    RNG.with(|rng| rng.f64())
}
