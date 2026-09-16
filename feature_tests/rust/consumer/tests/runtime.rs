//! Runtime behaviour of the generated safe API, exercised without `unsafe`.
#![forbid(unsafe_code)]

use std::sync::Mutex;

use diplomat_rust_backend_generated::{Bytes, Counter, Message, Mode, Numbers, SliceView};

/// `Counter` reports destruction through a process-wide probe, so the tests that
/// read it must not interleave with one another.
static COUNTER_PROBE: Mutex<()> = Mutex::new(());

fn counter_probe() -> std::sync::MutexGuard<'static, ()> {
    COUNTER_PROBE
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

#[test]
fn owner_side_destruction_runs_exactly_once() {
    let _serial = counter_probe();
    Counter::reset_drop_count();

    {
        let mut counter = Counter::new();
        let identity = counter.identity();
        counter.increment();
        counter.increment();
        counter.increment();
        assert_eq!(counter.get(), 3);
        assert_eq!(counter.add(Some(2)), Some(5));
        assert_eq!(counter.add(None), None);

        let snapshot = counter.snapshot();
        assert_eq!(snapshot.value, 5);
        assert!(snapshot.active);
        assert_eq!(snapshot.mode, Mode::Active);
        assert_eq!(counter.maybe_snapshot(true).unwrap().value, 5);
        assert!(counter.maybe_snapshot(false).is_none());

        {
            let child = counter.child();
            assert_eq!(child.get(), 5);
            assert_eq!(child.owner_identity(), identity);
        }
        assert!(counter.maybe_child(true).is_some());
        assert!(counter.maybe_child(false).is_none());
        {
            let mut child = counter.child_mut();
            child.set(9);
            assert_eq!(child.get(), 9);
        }

        let view = counter.view();
        assert!(counter.same_identity(&view));
    }

    assert_eq!(Counter::drop_count(), 1);
}

#[test]
fn shared_and_exclusive_views_reach_the_same_native_object() {
    let _serial = counter_probe();
    Counter::reset_drop_count();

    {
        let mut left = Counter::new();
        let mut right = Counter::new();
        right.add(Some(7));

        {
            let shared = right.view();
            assert_eq!(left.copy_value_from(&shared), 7);
        }
        {
            let mut exclusive = right.view_mut();
            left.exchange_values(&mut exclusive);
        }

        assert!(!left.same_identity(&right));
    }

    assert_eq!(Counter::drop_count(), 2);
}

#[test]
fn optional_owned_opaques_are_destroyed_after_construction() {
    let _serial = counter_probe();
    Counter::reset_drop_count();

    assert!(Counter::maybe_new(false).is_none());
    drop(Counter::maybe_new(true));

    assert_eq!(Counter::drop_count(), 1);
}

#[test]
fn borrowed_slices_read_write_and_fill() {
    let mut numbers = Numbers::new(&[1, 2, 3]);
    assert_eq!(numbers.sum(), 6);
    assert_eq!(numbers.values(), [1u32, 2, 3].as_slice());

    {
        let values = numbers.values_mut();
        assert_eq!(values.len(), 3);
        values[0] = 10;
    }
    assert_eq!(numbers.values(), [10u32, 2, 3].as_slice());

    let mut buffer = [0u32; 3];
    numbers.fill(&mut buffer);
    assert_eq!(buffer, [10, 2, 3]);
}

#[test]
fn borrowed_strings_are_unvalidated_bytes_and_utf8() {
    let message = Message::new(b"hello");
    assert_eq!(message.bytes(), &b"hello"[..]);
    assert_eq!(message.text(), "hello");
}

#[test]
fn owned_opaque_carries_a_type_level_lifetime() {
    let data = [1u8, 2, 3, 4];
    let view = SliceView::wrap(&data);

    assert!(!view.is_empty());
    assert_eq!(view.len(), 4);
    assert_eq!(view.get(0), 1);
    assert_eq!(view.get(9), 0);
    assert_eq!(view.sum(), 10);
    assert_eq!(view.data(), &data[..]);
}

#[test]
fn lifetime_bearing_value_struct_round_trips_by_value() {
    let data = [1u8, 2, 3, 4];
    let view = SliceView::wrap(&data);

    let fields = view.fields();
    assert_eq!(fields.count, 4);
    assert_eq!(fields.bytes, &data[..]);
    assert_eq!(Numbers::total(fields), 14);
}

#[test]
fn owned_slice_return_transfers_ownership() {
    let bytes = Bytes::make(5);
    assert_eq!(&bytes[..], &[0u8, 1, 2, 3, 4]);
    assert!(Bytes::make(0).is_empty());

    let joined = Bytes::join(&[1, 2], &[3, 4, 5]);
    assert_eq!(&joined[..], &[1, 2, 3, 4, 5]);
}
