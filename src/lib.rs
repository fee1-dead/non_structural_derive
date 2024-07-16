#[doc(hidden)]
pub fn check_send<T: Send>() {}

#[doc(hidden)]
pub fn check_sync<T: Sync>() {}

/// Derives `Send` and/or `Sync` without doing a "perfect" derive.
///
/// This uses behavior from the normal `derive` by adding the
/// `Send`/`Sync` bound to the generic parameters instead of fields.
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
/// This still errors if any of the fields are not `Send`/`Sync`:
///
/// ```compile_fail
/// # use non_structural_derive::non_structural_derive;
/// use std::rc::Rc;
///
/// #[non_structural_derive(Send)]
/// pub struct Foo<T>(Rc<T>);
/// ```
pub use non_structural_derive_macro::non_structural_derive;
