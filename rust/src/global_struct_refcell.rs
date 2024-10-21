use std::cell::RefCell;
const NUM_ITERATIONS: usize = 1000000;

#[derive(Default)]
struct GlobalStruct {
    pub value: i32,
}

// 전역 데이터
thread_local! {
    static GLOBAL_DATA: RefCell<GlobalStruct> = RefCell::new(GlobalStruct { value: 0 });
}

fn increment_global() {
    for i in 0..NUM_ITERATIONS {
        if i % 2 == 0 {
            GLOBAL_DATA.with(|data| {
                data.borrow_mut().value += 1; // RefCell을 통해 값 증가
            });
        }
    }
}

pub fn run() {
    increment_global();
    // GLOBAL_DATA.with(|data| {
    //     println!("Final value: {}", data.borrow().value);
    // });
}
