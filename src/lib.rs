//! # SOL_PRNG
//! A simple functional pseudo random number generator.
//! No mutability. Relies on shadowing as of now.
//! 
//! ## Functions:
//! ```rust
//! 
//! let r = Rand::new(42);
//! 
//! // -> true || false
//! r.next_bool();
//! 
//! // -> i32
//! r.next_i32();
//! 
//! // -> u32
//! r.next_u32();
//! 
//! // -> i64
//! r.next_i64();
//! 
//! // -> u64
//! r.next_u64();
//! 
//! // -> u32 in inclusive range (from, to)
//! r.next_ranged_u32(42, u32::MAX);
//! 
//! // -> u64 in inclusive range (from, to)
//! r.next_ranged_u64(42, u64::MAX);
//! 
//! ```
//! 
//! ## Usage:
//! ```rust
//! 
//! // Create an initial Rand. For non-determinism use something like the system
//! // time as a seed.
//! let r = Rand::new(42);
//! 
//! // Shadows previous Rand, creates n (holds your result).
//! let (r, n) = r.next_i32();
//! println!("n: {}", n);
//! 
//! // Shadows previous Rand, shadows n (holds your new result).
//! let (r, n) = r.next_i32();
//! println!("n: {}", n);
//! 
//! // It does not matter the type of the last generated value.
//! let (r, n) = r.next_bool();
//! println!("n: {}", n);
//! 
//! let (r, n) = r.next_u64();
//! println!("n: {}", n);
//! 
//! let (r, n) = r.next_ranged_u32(1, 255);
//! println!("n: {}", n);
//! 
//! ```
//!

/// Describes the state of a Rand generator. Member functions do not alter it's
/// fields, but return a new Rand instead.
pub struct Rand {
    curr_seed: u32,
    next_seed: u32,
}

impl Rand {
    fn secret_u32(seed: u32) -> u32 {
        let mut n: u32 = seed;
        n = (n ^ 61) ^ (n.wrapping_shr(16));
        n = n.wrapping_add(n.wrapping_shl(3));
        n = n ^ (n.wrapping_shr(4));
        n = n.wrapping_mul(0x27d4eb2d);
        n = n ^ (n.wrapping_shr(15));
        n
    }

    fn secret_u64(seed: u32) -> u64 {
        let mut n: u64 = u64::from(seed);
        n = (n ^ 61) ^ (n.wrapping_shr(16));
        n = n.wrapping_add(n.wrapping_shl(3));
        n = n ^ (n.wrapping_shr(4));
        n = n.wrapping_mul(0x27d4eb2d);
        n = n ^ (n.wrapping_shr(15));
        n
    }

    fn secret_bool(seed: u32) -> bool {
        (Rand::secret_u32(seed)) < (u32::MAX / 2)
    }

    fn new(seed: u32) -> Self {
        Self {
            curr_seed: seed,
            next_seed: Rand::secret_u32(seed),
        }
    }
}

impl Rand {

    fn err_switch() -> String {
        "Function next_ranged_* had parameter `from` greater than parameter
        `to`. So to not panic, the parameters have been switched.
         Do fix this.".replace("        ", " ")
    }

    /// Generates a tuple with the next Rand with a new seed and a bool 
    /// generated from the current seed.
    pub fn next_bool(&self) -> (Rand, bool) {
        (Rand::new(self.next_seed), Rand::secret_bool(self.next_seed))
    }

    /// Generates a tuple with the next Rand with a new seed and an i32 
    /// generated from the current seed.
    pub fn next_i32(&self) -> (Rand, i32) {
        let is_negative = Rand::secret_bool(self.curr_seed);
        let result = if is_negative {
            -((Rand::secret_u32(self.next_seed) as i32).abs())
        } else {
            (Rand::secret_u32(self.next_seed) as i32).abs()
        };
        (Rand::new(self.next_seed), result)
    }

    /// Generates a tuple with the next Rand with a new seed and an i64 
    /// generated from the current seed.
    pub fn next_i64(&self) -> (Rand, i64) {
        let is_negative = Rand::secret_bool(self.curr_seed);
        let result = if is_negative {
            -((Rand::secret_u64(self.next_seed) as i64).abs())
        } else {
            (Rand::secret_u64(self.next_seed) as i64).abs()
        };
        (Rand::new(self.next_seed), result)
    }

    /// Generates a tuple with the next Rand with a new seed and an u32 
    /// generated from the current seed.
    pub fn next_u32(&self) -> (Rand, u32) {
        (Rand::new(self.next_seed), Rand::secret_u32(self.next_seed))
    }

    /// Generates a tuple with the next Rand with a new seed and an u64 
    /// generated from the current seed.
    pub fn next_u64(&self) -> (Rand, u64) {
        (Rand::new(self.next_seed), Rand::secret_u64(self.next_seed))
    }

    /// Generates a tuple with the next Rand with a new seed and an u32 
    /// generated from the current seed that is in the provided inclusive range.
    /// The parameter `from` should be greater than `to`, otherwise this will
    /// issue a warning and switch the parameters.
    pub fn next_ranged_u32(&self, from: u32, to: u32) -> (Rand, u32) {
        let should_switch = from > to;
        if should_switch {
            dbg!(Rand::err_switch());
        };
        let (r, n) = self.next_u32();
        let (from, to) = if should_switch {
            (to, from)
        } else {
            (from, to)
        };
        let result = from + (n) % ((to - from) + 1);
        (r, result)
    }

    /// Generates a tuple with the next Rand with a new seed and an u64 
    /// generated from the current seed that is in the provided inclusive range.
    /// The parameter `from` should be greater than `to`, otherwise this will
    /// issue a warning and switch the parameters.
    pub fn next_ranged_u64(&self, from: u64, to: u64) -> (Rand, u64) {
        let should_switch = from > to;
        if should_switch {
            dbg!(Rand::err_switch());
        };
        let (r, n) = self.next_u64();
        let (from, to) = if should_switch {
            (to, from)
        } else {
            (from, to)
        };
        let result = from + (n) % ((to - from) + 1);
        (r, result)
    }
}
