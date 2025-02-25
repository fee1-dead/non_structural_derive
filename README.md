# `non_structural_derive`

A `derive`-macro replacing the builtin auto-trait implementation with a manual *non-recursive* one:

```rust
struct Foo<T> {
    a: Bar<T>,
    b: Baz,
}

// The builtin auto-trait impl would look like this:
unsafe impl<T> Send for Foo<T>
where
    Bar<T>: Send,
    Baz: Send,
{}

// `#[non_structural_derive(Send)]` emits the the following instead:
unsafe impl<T: Send> Send for Foo<T> {}
fn _check_bound<T: Send>() {}
fn _validate_fields<T: Send>() {
    _check_bound::<Bar<T>>();
    _check_bound::<Baz>();
}
```

## Why would you want this?

The code emitted by `non_structural_derive` is strictly weaker than the builtin auto-trait impl.
It may however improve compile times and avoid overflow errors for very deeply nested types.
