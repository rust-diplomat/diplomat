impl Drop for ffi::CountedOpaque {
    fn drop(&mut self) {
        self.0.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
    }
}

#[diplomat::bridge]
pub mod ffi {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    /// Counts how many distinct `CountedOpaque` objects are instanciated
    #[diplomat::opaque]
    pub struct Counter(Arc<AtomicUsize>);

    impl Counter {
        pub fn new() -> Box<Counter> {
            Box::new(Self(Arc::new(AtomicUsize::new(0))))
        }

        pub fn count(&self) -> usize {
            self.0.load(Ordering::SeqCst)
        }
    }

    #[diplomat::opaque]
    pub struct CountedOpaque(pub(crate) Arc<AtomicUsize>);

    impl CountedOpaque {
        pub fn new(counter: &Counter) -> Box<CountedOpaque> {
            counter.0.fetch_add(1, Ordering::SeqCst);
            Box::new(Self(counter.0.clone()))
        }
    }

    /// "Ownership is a delicous dish."
    /// — OwnershipEater
    #[diplomat::opaque]
    pub struct OwnershipEater(Option<Box<CountedOpaque>>);

    impl OwnershipEater {
        pub fn new() -> Box<OwnershipEater> {
            Box::new(Self(None))
        }

        // FIXME: not supported by C++ backend

        // pub fn take_opaque(&mut self, opaque: Box<CountedOpaque>) {
        //     self.0 = Some(opaque);
        // }

        // pub fn take_opaque_opt(&mut self, opaque_opt: Option<Box<CountedOpaque>>) {
        //     self.0 = opaque_opt;
        // }
    }
}
