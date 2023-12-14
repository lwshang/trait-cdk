# Simulations of Trait Bound Canister Dev

This project contains various implementation for the same canister interface [Counter](counter.did).

## Build and test

```
dfx start --clean --background
dfx deploy
dfx canister call <CANISTER_NAME> read
dfx canister call <CANISTER_NAME> inc
dfx canister call <CANISTER_NAME> read
...
dfx stop
```

## Canisters

### unit_struct

The baseline implementation which illustrates how the macro works.

### build_script

Instead of calling macro, we may want to use `build.rs` which generate the binding in `OUT_DIR`.

If we want to allow bindgen configuration, `build.rs` is more convenient than macro.

### stateful

The struct has state and the trait required methods take `&self` or `&mut self` in argument.

**Note**: Depends on `once_cell` crate to make sure that only one instance of the `Canister`.

### async_trait

The approach can work well with async.
