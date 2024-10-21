use std::sync::{Arc, RwLock};
const NUM_ITERATIONS: usize = 1000000;
#[derive(Default)]
struct GlobalStruct {
    value: i32,
}

// 전역 데이터
lazy_static::lazy_static! {
    static ref GLOBAL_DATA: Arc<RwLock<GlobalStruct>> = Arc::new(RwLock::new(GlobalStruct::default()));

}

fn increment_global() {
    let mut a: i32 = 0;
    for _ in 0..NUM_ITERATIONS {
        a = GLOBAL_DATA.read().unwrap().value;

        // data.value += 1;
    }
}

pub fn run() {
    increment_global();
    // let final_value = GLOBAL_DATA.read().unwrap(); // RwLock을 사용하여 읽기 잠금
    // println!("Final value: {}", final_value.value);
}
