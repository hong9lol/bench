use std::sync::{Arc, RwLock};
const NUM_ITERATIONS: usize = 1000000;

lazy_static::lazy_static! {
    static ref GLOBAL_VAR: Arc<RwLock<i32>> = Arc::new(RwLock::new(0));
}
fn increment_global() {
    for _ in 0..NUM_ITERATIONS {
        let mut data = GLOBAL_VAR.write().unwrap();
        *data += 1;
    }
}

pub fn run() {
    increment_global();
    // let final_value = GLOBAL_VAR.read().unwrap(); // RwLock을 사용하여 읽기 잠금
    // println!("Final value: {}", final_value);
}
