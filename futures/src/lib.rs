pub use futures_core::Future;

pub struct Foo;

impl Future for Foo {
    type Output = usize;

    fn poll(&mut self) -> Option<Self::Output> {
        Some(5)
    }
}
