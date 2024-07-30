# Lifetimes


Diplomat *is* able to safely handle methods that do complex borrowing, as long as the lifetimes are fully specified (i.e., not elided).

In general, Diplomat attempts to follow language norms when it comes to patterns akin to borrowing. In C++, for example, the norm is to document the behavior of a method, which is what Diplomat does. However, in JS, the norm is to not have use-after-frees, which Diplomat attempts to achieve by stashing extra references to borrowed-from objects.


For example, let's take this iterator API:

```rust
#[diplomat::bridge]
mod ffi {

    #[diplomat::opaque]
    pub struct MyFancyVec(Vec<u32>); // not very fancy

    pub struct MyFancyIterator<'a>(std::slice::Iter<'a, u32>);


    impl MyFancyVec {
        pub fn new(count: usize) -> Box<MyFancyVec> {
            // make a random vector, this is an example
            let vec = (5..(count + 5)).collect();
            Box::new(MyFancyVec(vec));
        }

        pub fn iter<'a>(&'a self) -> Box<MyFancyIterator<'a>> {
            Box::new(MyFancyIterator(self.0.iter()))
        }
    }

    impl<'a> MyFancyIterator<'a> {
        fn next(&mut self) -> Option<u32> {
            self.0.next().copied()
        }
    }
}
```

It's crucial the return type of `MyFancyVec::iter()` is not held on to longer than the `MyFancyVec` it came from.

In C++, this will produce documentation that looks like the following:

```
Lifetimes: `self` must live at least as long as the output.
```


On the other hand, in JS, the JS object wrapping `MyFancyIterator` will internally stash a reference to the `MyFancyVec`, which will be noticed by the GC, keeping the vector alive as long as needed.


This also works with non-opaque structs; you can have a function that takes in a struct where one field has a lifetime, and returns a different struct with a similar property, and Diplomat will document or GC-link the appropriate fields.
