// functions3.rs
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a hint.

use std::time::{Duration, Instant};
fn main() {
    let start = Instant::now();
    call_me(30000);
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
