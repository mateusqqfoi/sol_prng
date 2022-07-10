#![allow(dead_code)]
/// ## Usage:
///```rust
/// 
/// // make a starting Rand
/// let starting_rand = Rand::new(42);
///
/// // generate your next Rand and a value, in this case an i32
/// let (r, n) = r.next_i32_ranged(1, 10);
/// println!("Random number on range 1-10: {}", n);
///
/// // to keep using, shadow the last Rand with a new one,
/// // returned from next_* functions.
/// let (r, n) = r.next_i32_ranged(1, 10);
/// println!("Random number on range 1-10: {}", n);
/// let (r, n) = r.next_i32_ranged(1, 10);
/// println!("Random number on range 1-10: {}", n);
///```
pub struct Rand {
    seed: i32,
    value: i32,
}

impl Rand {
    const HALF_I32: i32 = i32::MAX / 2;

    /// Generates a new Rand with the provided seed.
    pub fn new(seed: i32) -> Self {
        Self {
            seed,
            value: Rand::gen(seed),
        }
    }

    fn gen(seed: i32) -> i32 {
        let mut n: i32 = seed;

        n = (n ^ 61) ^ (n.wrapping_shr(16));
        n = n.wrapping_add(n.wrapping_shl(3));
        n = n ^ (n.wrapping_shr(4));
        n = n.wrapping_mul(0x27d4eb2d);
        n = n ^ (n.wrapping_shr(15));
        n = n.wrapping_sub(Rand::HALF_I32);

        n
    }
}

impl Rand {
    /// Generates a new Rand pre-seeded and a pseudo random i32.
    pub fn next_i32(&self) -> (Rand, i32) {
        let seed = self.value;
        let value = Rand::gen(seed);
        (Rand { seed, value }, value)
    }

    /// Generates a new Rand pre-seeded and a pseudo random bool.
    pub fn next_bool(&self) -> (Rand, bool) {
        let (r, n) = self.next_i32();
        (r, n < 0)
    }

    /// Generates a new Rand pre-seeded and a pseudo random i32 
    /// in provided range.
    pub fn next_i32_ranged(&self, from: i32, to: i32) -> (Rand, i32) {
        let (r, n) = self.next_i32();
        let result = from + (n.abs()) % ((to - from).abs() + 1);
        (r, result)
    }
}
