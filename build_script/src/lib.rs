include!(concat!(env!("OUT_DIR"), "/counter.rs"));

use std::cell::Cell;

thread_local! {
    static COUNTER: Cell<u64> = Cell::new(0);
}

struct Canister;

impl Counter for Canister {
    fn inc() {
        COUNTER.set(COUNTER.get() + 1);
    }

    fn read() -> u64 {
        COUNTER.get()
    }
}
