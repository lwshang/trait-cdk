use std::path::Path;

static BINDING: &str = "trait Counter {
    fn inc();
    fn read() -> u64;
}

#[export_name = \"canister_update inc\"]
fn __canister_method_inc() {
    ic_cdk::setup();
    ic_cdk::spawn(async {
        let _result = Canister::inc();
        ic_cdk::api::call::reply(())
    });
}

#[export_name = \"canister_query read\"]
fn __canister_method_read() {
    ic_cdk::setup();
    ic_cdk::spawn(async {
        let result = Canister::read();
        ic_cdk::api::call::reply((result,))
    });
}";

fn main() {
    println!("cargo:rerun-if-changed=../counter.did");
    println!("cargo:rerun-if-changed=build.rs");
    // ic_cdk_bindgen::generate_provider("../counter.did").unwrap()
    // We can also introduce Config similar to
    // https://docs.rs/prost-build/latest/prost_build/struct.Config.html#method.compile_protos
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let path = Path::new(&out_dir).join("counter.rs");
    std::fs::write(path, BINDING).unwrap();
}
