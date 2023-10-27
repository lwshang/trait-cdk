// ic_cdk::generate!({path: "../../counter.did", service : Canister, trait: Counter});

// Expand Start
#[async_trait::async_trait]
trait Counter {
    async fn inc();
    async fn read() -> u64;
}

#[export_name = "canister_update inc"]
fn __canister_method_inc() {
    ic_cdk::setup();
    ic_cdk::spawn(async {
        let _result = Canister::inc().await;
        ic_cdk::api::call::reply(())
    });
}

#[export_name = "canister_query read"]
fn __canister_method_read() {
    ic_cdk::setup();
    ic_cdk::spawn(async {
        let result = Canister::read().await;
        ic_cdk::api::call::reply((result,))
    });
}
// Expand End

use std::cell::Cell;

thread_local! {
    static COUNTER: Cell<u64> = Cell::new(0);
}

struct Canister;

#[async_trait::async_trait]
impl Counter for Canister {
    async fn inc() {
        COUNTER.set(COUNTER.get() + 1);
    }

    async fn read() -> u64 {
        COUNTER.get()
    }
}
