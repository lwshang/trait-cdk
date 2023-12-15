// ic_cdk::generate!({path: "../../counter.did", service : Canister, trait: Counter});

// Expand Start
trait Counter {
    async fn inc(&mut self);
    fn read(&self) -> u64;
}

#[export_name = "canister_update inc"]
fn __canister_method_inc() {
    ic_cdk::setup();
    ic_cdk::spawn(async {
        let _result = CANISTER.with(|c| c.borrow_mut().inc().await);
        ic_cdk::api::call::reply(())
    });
}

#[export_name = "canister_query read"]
fn __canister_method_read() {
    ic_cdk::setup();
    ic_cdk::spawn(async {
        let result = CANISTER.with(|c| c.borrow().read());
        ic_cdk::api::call::reply((result,))
    });
}

thread_local! {
    static CANISTER: ::std::cell::RefCell<Canister> = ::std::cell::RefCell::new(Canister::default());
}
// Expand End

#[derive(Default)]
struct Canister {
    counter: u64,
}

impl Counter for Canister {
    async fn inc(&mut self) {
        self.counter += 1;
    }

    fn read(&self) -> u64 {
        self.counter
    }
}
