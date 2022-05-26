#[diplomat::bridge]
mod ffi {
    #[diplomat::opaque]
    struct RefList<'a>( (&'a i32, Option<Box<Self>>));

    impl<'b> RefList<'b> {
        pub fn node(data: &'b i32) -> Box<Self> {
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
