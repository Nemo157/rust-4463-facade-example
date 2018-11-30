use futures::{Future, Foo};

// Error: `Foo` is an item from a private dependency in a public API
pub fn foo() -> Foo {
    Foo
}

// Ok: `Future` is originally from a public dependency, just re-exported
// through a private dependency
pub fn bar() -> impl Future {
    Foo
}
