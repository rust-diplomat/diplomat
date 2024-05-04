#[diplomat::bridge]
mod ffi {
    #[diplomat::opaque]
    #[derive(Clone, Copy)]
    struct RefListParameter;

    #[diplomat::opaque]
    #[diplomat::attr(disable, kotlin)]
    #[derive(Clone)]
    struct RefList<'a>((&'a RefListParameter, Option<Box<Self>>));

    impl<'b> RefList<'b> {
        #[diplomat::attr(supports = named_constructors, named_constructor)]
        pub fn node(data: &'b RefListParameter) -> Box<Self> {
            Box::new(RefList((data, None)))
        }

        pub fn extend(&mut self, other: &Self) {
            match self.0 .1.as_mut() {
                Some(tail) => tail.extend(other),
                None => self.0 .1 = Some(Box::new(other.clone())),
            }
        }
    }
}
