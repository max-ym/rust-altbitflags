//! This crate contains macros that makes creating bitfields more intuitive.
//!
//! It is an alternative way of what 'Rust Compiler Team Bitflags' do.
//! It is not similar to C bitflags but it may be a bit easier to read and use
//! in some cases. Furthermore, this crate does not conflict with
//! 'Rust Compiler Team Bitflags' and you can use both as it must be okay.
//!
//! # Example
//! ```rust
//! # #[macro_use]
//! # extern crate altbitflags;
//! #
//! // Create some structure which will act like a bitfield structure.
//! struct Something(i64);
//!
//! impl Something {
//!
//!     // Create ReadWrite flag called 'present' (or shorthand 'p').
//!     // To set a value use 'set_p' or 'set_present'.
//!     // Flag position is bit number 0.
//!     flag_rw!(p, set_p, present, set_present, 0);
//!
//!     // Create ReadOnly flag 'extended' (and 'e') at position 1.
//!     flag_ro!(e, extended, 1);
//!
//!     // Shorter version of a flag_rw! macro.
//!     flag_rw!(a, set_a, 2);
//!
//!     // Shorter version of a flag_ro! macro.
//!     flag_ro!(b, 3);
//! }
//!
//! fn main() {
//!     let mut something = Something(0);
//!
//!     // This means the same thing:
//!     if (something.p()) { /* ... */ }
//!     if (something.present()) { /* ... */ }
//!
//!     // As does this:
//!     something.set_present(true);
//!     something.set_p(true);
//! }
//! ```

/// Create read-only flag.
///
/// It accepts a name argument, optional full name argument and a number,
/// which indicates the bit number with required flag.
///
/// Later, the given flag can be accessed either by short or full name
/// in a form of a normal function.
///
/// It will work only if it's put in a structure implementation
/// where integer number with bit fields can be accessed with `self.0`.
///
/// ReadOnly flags accepts `self.0` to be of any integer type.
///
/// # Example
///
/// ```
/// # #![allow(unused_parens)]
/// #[macro_use]
/// extern crate altbitflags;
///
/// struct Something(i64);
///
/// impl Something {
///     flag_ro!(present, 0);
///     flag_ro!(tx, 1);
///     flag_ro!(e, extended, 2);
///
///     // It is okay to define different functions for a single bit:
///     flag_ro!(some, 3);
///     flag_ro!(some_bit, 3);
/// }
///
/// fn main() {
///     let mut something = Something(0);
///
///     if (something.present()) { /* ... */ }
///     if (something.tx()) { /* ... */ }
///
///     // These do the same thing:
///     if (something.e()) { /* ... */ }
///     if (something.extended()) { /* ... */ }
/// }
/// ```
#[macro_export]
macro_rules! flag_ro {
    ($name:ident, $pos:expr) => (
        #[inline="always"]
        pub fn $name(&self) -> bool { self.0 & (1 << $pos) != 0 }
    );

    ($name:ident, $full_name:ident, $pos:expr) => (
        flag_ro!($name, $pos);
        flag_ro!($full_name, $pos);
    );
}

/// Create read-write flag.
///
/// It accepts an name argument, optional full name argument and a number,
/// which indicates the bit number with required flag.
///
/// Later, the given flag can be accessed either by short or full name
/// like with a normal function.
///
/// It will work only if it's put in a structure implementation
/// where integer number with fields can be accessed with `self.0`.
///
/// Currently, `self.0` must be of type `i64`.
///
/// # Example
///
/// ```
/// # #![allow(unused_parens)]
/// #[macro_use]
/// extern crate altbitflags;
///
/// struct Something(i64);
///
/// impl Something {
///     flag_rw!(present, set_present, 0);
///     flag_rw!(e, set_e, extended, set_extended, 1);
///
///     // It is okay to define different functions for a single bit:
///     flag_rw!(a, set_a, 2);
///     flag_rw!(b, set_b, 2);
/// }
///
/// fn main() {
///     let mut something = Something(0);
///
///     if (something.present()) { /* ... */ }
///     if (something.e()) { /* ... */ }
///
///     // These do the same thing:
///     something.set_e(true);
///     something.set_extended(true);
/// }
/// ```
#[macro_export]
macro_rules! flag_rw {
    ($name:ident, $set_name:ident, $pos:expr) => (
        flag_ro!($name, $pos);

        #[inline="always"]
        pub fn $set_name(&mut self, b: bool) {
            let x = b as i64;
            self.0 ^= (-x ^ self.0) & (1 << $pos);
        }
    );

    ($name:ident, $set_name:ident,
    $full_name:ident, $full_set_name:ident,
    $pos:expr) => (
        flag_rw!($name, $set_name, $pos);
        flag_rw!($full_name, $full_set_name, $pos);
    );
}

#[cfg(test)]
mod test;
