use std::sync::atomic::{AtomicI32, Ordering};
const NUM_ITERATIONS: usize = 1000000;

#[derive(Default)]
struct GlobalStruct {
    value: AtomicI32,
}

// 전역 데이터
static GLOBAL_DATA: GlobalStruct = GlobalStruct {
    value: AtomicI32::new(0),
};

fn increment_global() {
    for i in 0..NUM_ITERATIONS {
        if i % 2 == 0 {
            GLOBAL_DATA.value.fetch_add(1, Ordering::Relaxed);
        }
    }
}

pub fn run() {
    increment_global();
    // println!("Final value: {}", GLOBAL_DATA.value.load(Ordering::Relaxed));
}
