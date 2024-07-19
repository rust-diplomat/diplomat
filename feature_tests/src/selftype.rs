#[diplomat::bridge]
mod ffi {
    #[diplomat::opaque]
    struct RefListParameter;

    #[diplomat::opaque]
    struct RefList<'a>((&'a RefListParameter, Option<Box<Self>>));

    impl<'b> RefList<'b> {
        #[diplomat::attr(*, named_constructor)]
        pub fn node(data: &'b RefListParameter) -> Box<Self> {
            Box::new(RefList((data, None)))
        }

        // pub fn extend(&mut self, other: Self) {
        //     match self.next.as_mut() {
        //         Some(tail) => tail.extend(other),
        //         None => self.next = Some(Box::new(other)),
        //     }
        // }
    }
}
