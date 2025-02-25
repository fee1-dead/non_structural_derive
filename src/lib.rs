/// Derives an auto-trait without doing a "perfect" derive.
///
/// This uses behavior from the normal `derive` by adding the
/// bound to the generic parameters instead of fields.
///
/// ## Examples
///
/// Use this on a recursive type:
///
/// ```
/// # use non_structural_derive::non_structural_derive;
/// use std::marker::PhantomData;
/// #[non_structural_derive(Send)]
/// pub struct Foo<T>(Box<Option<Foo<T>>>, PhantomData<T>);
/// ```
///
/// This still errors if any of the fields do not implement the auto-trait:
///
/// ```compile_fail
/// # use non_structural_derive::non_structural_derive;
/// use std::rc::Rc;
///
/// #[non_structural_derive(Send)]
/// pub struct Foo<T>(Rc<T>);
/// ```
///
/// ## Supported traits
///
/// This crate currently supports all stable auto-traits: `Send`, `Sync`, `Unpin`,
/// `UnwindSafe`, `RefUnwindSafe`.
pub use non_structural_derive_macro::non_structural_derive;
