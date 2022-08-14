//! Interact with an enum type via traits.

#[cfg(feature = "derive")]
pub use typeenum_derive::HasVariant;

/// Signifies this type functions like an enum with variant `T`. This
/// means you can create `Self` from `T`, and given a `Self` you can
/// try to convert it into a `T`.
pub trait HasVariant<T>: From<T> {
    fn get(&self) -> Option<&T>;
    fn get_mut(&mut self) -> Option<&mut T>;
}
