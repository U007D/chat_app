#![allow(non_snake_case, clippy::result_unwrap_used, clippy::wildcard_imports)]
use super::*;
use assert2::assert;
use dashmap::DashSet;
use std::panic::catch_unwind;

#[test]
fn addr__repeated_calls_give_unique_results() {
    // Given a `UniqueAddr` instance
    let n_attempts = 10_000;
    let set = DashSet::new();
    let mut res = Vec::new();
    let uniq = UniqueAddr::new();

    // When `addr()` is called repeatedly
    (0..n_attempts).for_each(|_| res.push(set.insert(uniq.addr())));

    // Then every `MemoryTransportAddr` returned was unique
    assert!(res.iter().all(|el| *el), "{:?}", res);
}

#[test]
fn addr__unique_addr_panics_on_overflow() {
    // Given a `UniqueAddr` instance set to max addr
    let uniq = UniqueAddr(Mutex::new(usize::max_value()));

    // When `addr()` is called
    let res = catch_unwind(|| uniq.addr());

    // Then it should have overflowed
    assert!(res.is_err(), "{:?}", res);
}
