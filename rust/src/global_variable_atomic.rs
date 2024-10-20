use std::sync::atomic::{AtomicI32, Ordering};
const NUM_ITERATIONS: usize = 1000000;

static GLOBAL_VAR: AtomicI32 = AtomicI32::new(0);

fn increment_global() {
    for _ in 0..NUM_ITERATIONS {
        GLOBAL_VAR.fetch_add(1, Ordering::Relaxed);
    }
}

pub fn run() {
    increment_global();
    // println!("Final value: {}", GLOBAL_VAR.load(Ordering::Relaxed));
}
