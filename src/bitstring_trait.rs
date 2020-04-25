/// A trait that defines the interface for bitstring manipulation
pub trait BitString {
    /// Returns the bit in the specified position or None if the position is out of bounds.
    fn get_bit(&self, pos: usize) -> Option<u8>;

    /// Sets the bit in the specified position and returns true if it was
    /// set successfully, otherwise false.
    fn set_bit(&mut self, pos: usize) -> bool;

    /// Clears the bit in the specified position and returns true if it
    /// was cleared successfully, otherwise false.
    fn clear_bit(&mut self, pos: usize) -> bool;

    /// Toggles the bit in the specified position and returns true if it
    /// was toggled successfully, otherwise false.
    fn toggle_bit(&mut self, pos: usize) -> bool;

    /// Clears the entire underlying bitstring.
    fn clear_string(&mut self);
}

#[inline]
fn bits<T>() -> usize {
    std::mem::size_of::<T>() * 8
}

macro_rules! bitstring_impl {
    ($type:ty) => {
        impl BitString for $type {
            fn get_bit(&self, pos: usize) -> Option<u8> {
                let bits = bits::<$type>();
                if pos < bits {
                    let res = ((*self >> pos) & 1) as u8;
                    Some(res)
                } else {
                    None
                }
            }

            fn set_bit(&mut self, pos: usize) -> bool {
                let bits = bits::<$type>();
                if pos < bits {
                    *self |= 1 << pos;
                    true
                } else {
                    false
                }
            }

            fn clear_bit(&mut self, pos: usize) -> bool {
                let bits = bits::<$type>();
                if pos < bits {
                    *self &= !(1 << pos);
                    true
                } else {
                    false
                }
            }

            fn toggle_bit(&mut self, pos: usize) -> bool {
                let bits = bits::<$type>();
                if pos < bits {
                    *self ^= 1 << pos;
                    true
                } else {
                    false
                }
            }

            fn clear_string(&mut self) {
                *self = 0;
            }
        }
    };
}

bitstring_impl!(i8);
bitstring_impl!(u8);
bitstring_impl!(i16);
bitstring_impl!(u16);
bitstring_impl!(i32);
bitstring_impl!(u32);
bitstring_impl!(i64);
bitstring_impl!(u64);
bitstring_impl!(i128);
bitstring_impl!(u128);
bitstring_impl!(usize);
