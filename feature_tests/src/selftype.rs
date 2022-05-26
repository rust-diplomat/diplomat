#[diplomat::bridge]
mod ffi {
    #[diplomat::opaque]
    struct RefList<'a> {
        data: &'a i32,
        next: Option<Box<Self>>,
    }

    impl<'b> RefList<'b> {
        pub fn node(data: &'b i32) -> Box<Self> {
            Box::new(RefList { data, next: None })
        }

        // pub fn extend(&mut self, other: Self) {
        //     match self.next.as_mut() {
        //         Some(tail) => tail.extend(other),
        //         None => self.next = Some(Box::new(other)),
        //     }
        // }
    }
}
