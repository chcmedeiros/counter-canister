use ic_cdk_macros::*;

static mut COUNTER: u64 = 0;

#[init]
fn init() {
    unsafe {
        COUNTER = 0;
    }
}

#[update]
fn increment() {
    unsafe {
        COUNTER += 1;
    }
}

#[query]
fn get() -> u64 {
    unsafe {
    COUNTER
    }
}

#[update]
fn set(input: u64) {
    unsafe {
        COUNTER =input;
    }
}
