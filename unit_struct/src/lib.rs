// ic_cdk::generate!({path: "../../counter.did", service : Canister, trait: Counter});

// Expand Start
trait Counter {
    fn inc();
    fn read() -> u64;
}

#[export_name = "canister_update inc"]
fn __canister_method_inc() {
    ic_cdk::setup();
    ic_cdk::spawn(async {
        let _result = Canister::inc();
        ic_cdk::api::call::reply(())
    });
}

#[export_name = "canister_query read"]
fn __canister_method_read() {
    ic_cdk::setup();
    ic_cdk::spawn(async {
        let result = Canister::read();
        ic_cdk::api::call::reply((result,))
    });
}
// Expand End

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
