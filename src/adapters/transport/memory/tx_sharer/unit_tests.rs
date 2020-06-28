#![allow(non_snake_case, clippy::result_unwrap_used, clippy::wildcard_imports)]
use super::*;
use assert2::assert;
use std::sync::mpsc::{channel, RecvTimeoutError};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[test]
fn lock__yields_sender_when_not_contended() {
    #[derive(Debug, Eq, PartialEq)]
    enum TestMsg {
        Spawned,
        AcquiredInner,
    }

    // Given a(n) (uncontended) `TxSharer`
    let (test_tx, test_rx) = channel();
    let sut = TxSharer::new(channel::<()>().0);

    // When the inner type is requested
    let _ = thread::spawn(move || {
        test_tx.send(TestMsg::Spawned).unwrap();
        let _res = sut.lock();
        test_tx.send(TestMsg::AcquiredInner).unwrap();
    });

    // Then, once the spawned thread has begun running,
    let sync_res = test_rx.recv_timeout(Duration::from_millis(50)).unwrap();
    assert!(sync_res == TestMsg::Spawned);

    // the request for `the inner resource will not block; (wait 50ms for notification `TxSharer`'s
    // inner resource has been acquired)
    let acq_res = test_rx.recv_timeout(Duration::from_millis(50)).unwrap();
    assert!(acq_res == TestMsg::AcquiredInner);
}

#[test]
fn lock__blocks_when_contended() {
    #[derive(Debug, Eq, PartialEq)]
    enum TestMsg {
        Spawned,
        AcquiredInner,
    }

    // Given a (contended) `TxSharer`
    let (test_tx, test_rx) = channel();
    let sut = Arc::new(TxSharer::new(channel::<()>().0));
    let sharer_ref = sut.clone();
    {
        let _lock = sut.lock();

        // When the inner type is requested
        let _ = thread::spawn(move || {
            test_tx.send(TestMsg::Spawned).unwrap();
            let _res = sharer_ref.lock();
            test_tx.send(TestMsg::AcquiredInner).unwrap();
        });

        // Then, once the spawned thread has begun running,
        let sync_res = test_rx.recv_timeout(Duration::from_millis(50)).unwrap();
        assert!(sync_res == TestMsg::Spawned);

        // the request for `the inner resource will block; (wait 50ms for timeout notification)
        let acq_res = test_rx.recv_timeout(Duration::from_millis(50)).unwrap_err();
        assert!(acq_res == RecvTimeoutError::Timeout);

        // Drop the contending lock
    }

    // due to lack of contention, the request for `the inner resource will now succeed; (wait 50ms
    // for notification `TxSharer`'s inner resource has been acquired)
    let acq_res = test_rx.recv_timeout(Duration::from_millis(50)).unwrap();
    assert!(acq_res == TestMsg::AcquiredInner);
}
