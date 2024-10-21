use std::cell::RefCell;
use std::rc::Rc;
const NUM_ITERATIONS: usize = 1000000;

thread_local! {
    pub static GLOBAL_VAR: Rc<RefCell<usize>> = Rc::new(RefCell::new(0));
}

fn increment_global() {
    for i in 0..NUM_ITERATIONS {
        if i % 2 == 0 {
            GLOBAL_VAR.with(|value| {
                *value.borrow_mut() += 1; // RefCell을 통해 증가
            });
        }
    }
}

pub fn run() {
    increment_global();
}
