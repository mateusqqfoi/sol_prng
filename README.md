# SOL_PRNG
A simple functional pseudo random number generator.
No mutability. Relies on shadowing as of now.

## Functions:
```rust

let r = Rand::new(42);

// -> true || false
r.next_bool();

// -> i32
r.next_i32();

// -> u32
r.next_u32();

// -> i64
r.next_i64();

// -> u64
r.next_u64();

// -> u32 in inclusive range (from, to)
r.next_ranged_u32(42, u32::MAX);

// -> u64 in inclusive range (from, to)
r.next_ranged_u64(42, u64::MAX);

```

## Usage:
```rust

// Create an initial Rand. For non-determinism use something like the system
// time as a seed.
let r = Rand::new(42);

// Shadows previous Rand, creates n (holds your result).
let (r, n) = r.next_i32();
println!("n: {}", n);

// Shadows previous Rand, shadows n (holds your new result).
let (r, n) = r.next_i32();
println!("n: {}", n);

// It does not matter the type of the last generated value.
let (r, n) = r.next_bool();
println!("n: {}", n);

let (r, n) = r.next_u64();
println!("n: {}", n);

let (r, n) = r.next_ranged_u32(1, 255);
println!("n: {}", n);

```
