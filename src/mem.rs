pub use core::{
    cell::{Cell, UnsafeCell},
    marker::{PhantomData, Unpin},
    mem::{
        align_of, forget, offset_of, replace, size_of, size_of_val, swap, take, transmute,
        transmute_copy, ManuallyDrop, MaybeUninit,
    },
    pin::{pin, Pin},
    ptr::{copy, copy_nonoverlapping, drop_in_place, write_bytes, NonNull},
};

/// Returns a byte slice representing the memory of the given value.
///
/// # Safety
///
/// The caller must ensure that `value` is valid for reads of `size_of_val(value)` bytes,
/// and that interpreting the memory as a byte slice is safe for the type `T`.
pub unsafe fn as_bytes<T>(value: &T) -> &[u8] {
    unsafe { crate::slice::from_raw_parts(value as *const T as *const u8, size_of_val(value)) }
}
