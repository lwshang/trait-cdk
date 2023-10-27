// ic_cdk::generate!({path: "../../counter.did", service : Canister, trait: Counter});

// Expand Start
trait Counter {
    fn inc(&mut self);
    fn read(&self) -> u64;
}

#[export_name = "canister_update inc"]
fn __canister_method_inc() {
    ic_cdk::setup();
    ic_cdk::spawn(async {
        let _result = CANISTER.lock().unwrap().inc();
        ic_cdk::api::call::reply(())
    });
}

#[export_name = "canister_query read"]
fn __canister_method_read() {
    ic_cdk::setup();
    ic_cdk::spawn(async {
        let result = CANISTER.lock().unwrap().read();
        ic_cdk::api::call::reply((result,))
    });
}

use once_cell::sync::Lazy;
use std::sync::Mutex;
static CANISTER: Lazy<Mutex<Canister>> = Lazy::new(|| Mutex::new(Default::default()));

// Expand End

#[derive(Default)]
struct Canister {
    counter: u64,
}

impl Counter for Canister {
    fn inc(&mut self) {
        self.counter += 1;
    }

    fn read(&self) -> u64 {
        self.counter
    }
}
