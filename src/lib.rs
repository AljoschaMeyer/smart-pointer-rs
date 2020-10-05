//! Traits for smart pointer functionality, i.e. shared ownership of values.
//!
//! The basic traits are `SmartPointer` and `SmartPointerMut` for basic creation, cloning,
//! obtaining ownership, borrowing and dereferencing. Further subtraits expose more specialized
//! functionality which is nonetheless applicable to any sort of smart pointer.
//!
//! These additional, specialized traits are being added to this crate as consumers need them. If
//! you would like to see traits for additional features, e.g. conversion with raw pointers,
//! [efficient borrows](https://docs.rs/triomphe/0.1.1/triomphe/struct.ArcBorrow.html),
//! [pointers directly to the data](https://docs.rs/triomphe/0.1.1/triomphe/struct.OffsetArc.html)
//! or [thin DST pointers](https://docs.rs/triomphe/0.1.1/triomphe/struct.ThinArc.html), open an
//! issue.

#![no_std]
// #![cfg_attr(feature = "unstable", coerce_unsized, dispatch_from_dyn)]
extern crate maybe_std as base;

use base::borrow::{Borrow, BorrowMut};
use base::ops::{Deref, DerefMut};
use base::fmt::Pointer;

/// The minimum amount of functionality common to all smart pointer types sharing ownership of a
/// value of type `T`. This trait only grants immutable access to the stored value, see
/// `SmartPointerMut` for mutable access and `TryIntoMut` for fallible conversion into a mutable
/// variant.
///
/// Note that most of the actual pointer functionality comes from the prerequisite traits.
///
/// Also note that this trait omits some functionality because it can only be expressed with
/// higher-kinded types, such as working with uninitialized memory, conversions to slices,
/// downcasting of `Any` values.
pub trait SmartPointer<T: ?Sized>: Sized + Clone + AsRef<T> + Borrow<T> + Deref<Target = T> + Pointer
// + CoerceUnsized<Ptr<U>> + DispatchFromDyn<Rc<U>> where T: Unsize<U>, U: ?Sized
{
    /// Construct a new smart pointer, containing the given value.
    fn new(t: T) -> Self;

    /// Try to obtain ownership of the wrapped value.
    ///
    /// This fails if there are other smart pointers wrapping the exact same value.
    fn try_unwrap(this: Self) -> Result<T, Self> where T: Sized;
}

/// A `SmartPointer` which beyond immutable access to the wrapped value also provides mutable
/// access via the `AsMut`, `BorrowMut` and `DerefMut` traits.
pub trait SmartPointerMut<T: ?Sized>: SmartPointer<T> + AsMut<T> + BorrowMut<T> + DerefMut<Target = T> {}

/// A `SmartPointer` which might grant mutable access, depending on run-time checks.
pub trait IntoMut<T: ?Sized>: SmartPointer<T> {
    type MutablePointer: SmartPointerMut<T> + Into<Self>;

    /// Check whether converting into a mutable version would succeed.
    fn can_make_mut(this: &Self) -> bool;

    /// Convert into a mutable version without performing runtime checks for upholding any
    /// invariants.
    ///
    /// Safety: Calling this is safe if and only if `can_make_mut` returns true.
    unsafe fn into_mut_unchecked(this: Self) -> Self::MutablePointer;

    /// Try converting into a mutable version of the pointer.
    fn into_mut(this: Self) -> Result<Self::MutablePointer, Self> {
        if IntoMut::can_make_mut(&this) {
            Ok(unsafe { IntoMut::into_mut_unchecked(this) })
        } else {
            Err(this)
        }
    }

    /// Obtain a mutable reference to the wrapped value without performing runtime checks for
    /// upholding any invariants.
    ///
    /// Safety: Calling this is safe if and only if `can_make_mut` returns true.
    unsafe fn get_mut_unchecked(this: &Self) -> &mut T;

    /// Try obtaining a mutable reference to the wrapped value.
    fn get_mut(this: &Self) -> Option<&mut T> {
        if IntoMut::can_make_mut(this) {
            Some(unsafe { IntoMut::get_mut_unchecked(this) })
        } else {
            None
        }
    }
}

// Might become trait:
//
// // fn as_ptr(this: &Self) -> *const T;
// fn into_raw(this: Self) -> *const T;
// unsafe fn from_raw(ptr: *const T) -> Self;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
