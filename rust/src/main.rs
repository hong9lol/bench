#[cfg(feature = "global_struct_arc_rwlock")]
mod global_struct_arc_rwlock;
#[cfg(feature = "global_struct_atomic")]
mod global_struct_atomic;
#[cfg(feature = "global_struct_refcell")]
mod global_struct_refcell;
#[cfg(feature = "global_variable_arc_rwlock")]
mod global_variable_arc_rwlock;
#[cfg(feature = "global_variable_atomic")]
mod global_variable_atomic;
#[cfg(feature = "global_variable_refcell")]
mod global_variable_refcell;

use std::time::{Duration, Instant};
const TEST_NUM: u32 = 1; // refcell issue
fn main() {
    let mut duration: Duration = Duration::new(0, 0);
    for _ in 0..TEST_NUM {
        let start = Instant::now();

        #[cfg(feature = "global_variable_arc_rwlock")]
        global_variable_arc_rwlock::run();
        #[cfg(feature = "global_variable_refcell")]
        global_variable_refcell::run();
        #[cfg(feature = "global_variable_atomic")]
        global_variable_atomic::run();
        #[cfg(feature = "global_struct_arc_rwlock")]
        global_struct_arc_rwlock::run();
        #[cfg(feature = "global_struct_refcell")]
        global_struct_refcell::run();
        #[cfg(feature = "global_struct_atomic")]
        global_struct_atomic::run();

        duration += start.elapsed();
    }
    println!("{:?}", duration / TEST_NUM);
}

// use std::cell::RefCell;

// const TEST_NUM: u32 = 100;
// use std::time::{Duration, Instant};
// fn main() {
//     // RefCell로 감싼 가변 변수 초기화
//     let mut duration: Duration = Duration::new(0, 0);
//     let counter = RefCell::new(0);

//     // 0부터 1,000,000까지 +1 하는 루프
//     for _ in 0..100 {
//         for _ in 0..1_000_000 {
//             let start = Instant::now();
//             // RefCell을 통해 값에 접근하고 증가
//             *counter.borrow_mut() += 1;
//             duration += start.elapsed();
//         }
//     }

//     println!("{:?}", duration / TEST_NUM);
//     // 최종 값 출력
//     println!("Final value: {}", *counter.borrow());
// }
