//! Interact with an enum type via traits.

pub use typeenum_derive::HasVariant;

pub trait HasVariant<T>: From<T> {
    fn get(&self) -> Option<&T>;
    fn get_mut(&mut self) -> Option<&mut T>;
}
