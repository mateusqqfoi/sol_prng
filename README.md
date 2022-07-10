# SOL_PRNG
A simple *functional pseudo random number generator*.
**No mutability**. Relies on shadowing as of now.

## Usage:
```rust 
// bring into scope
use sol_prng::Rand;

// make a starting Rand
let starting_rand = Rand::new(42);

// generate your next Rand and a value, in this case an i32
let (r, n) = r.next_i32_ranged(1, 10);
println!("Random number on range 1-10: {}", n);

// to keep using, shadow the last Rand with a new one,
// returned from next_* functions.
let (r, n) = r.next_bool();
println!("bool: {:?}", n);

// the type you previously generated through a next_* method (^ bool) 
// does not matter, just shadow normally.
let (r, n) = r.next_i32_ranged(1, 10);
println!("Random number on range 1-10: {}", n);
```