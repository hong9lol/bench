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
    for i in 0..NUM_ITERATIONS {
        if GLOBAL_DATA.read().unwrap().value % 2 == 0 {
            let a = 0;
        }
    }
}

pub fn run() {
    increment_global();
    // let final_value = GLOBAL_DATA.read().unwrap(); // RwLock을 사용하여 읽기 잠금
    // println!("Final value: {}", final_value.value);
}
