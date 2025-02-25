use non_structural_derive::non_structural_derive;

struct Nested<T>(T);

#[allow(unused)]
#[non_structural_derive(Send, Sync, Unpin, UnwindSafe, RefUnwindSafe)]
struct MyType<T> {
    value: Nested<T>,
}

fn main() {
    use std::panic::{RefUnwindSafe, UnwindSafe};
    fn impls_send<T: Send>() {}
    fn impls_sync<T: Sync>() {}
    fn impls_unpin<T: Unpin>() {}
    fn impls_unwind_safe<T: UnwindSafe>() {}
    fn impls_ref_unwind_safe<T: RefUnwindSafe>() {}
    impls_send::<MyType<u32>>();
    impls_sync::<MyType<u32>>();
    impls_unpin::<MyType<u32>>();
    impls_unwind_safe::<MyType<u32>>();
    impls_ref_unwind_safe::<MyType<u32>>();
}
