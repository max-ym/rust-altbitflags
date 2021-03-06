# altbitflags
If you need to create a lot of many-many-many bitfields in an intuitive way, check out some of these macro.

It is not a C-like bitfield structure generator. It's more like a bitmap where each bit has it's own name. If you need to change particular bits in integer numbers you may access them by functions that are generated with these macros.

Documentation can be found [here](https://max-ym.github.io/rust-altbitflags/docs/altbitflags/).

## Demo
```rust
// Make sure to write #[macro_use]!
#[macro_use]
extern crate altbitflags;

// Create some structure which will act like a bitfield structure.
struct Something(i64);

impl Something {

    // Create ReadWrite flag called 'present' (or shorthand 'p').
    // To set a value use 'set_p' or 'set_present'.
    // Flag position is bit number 0.
    flag_rw!(p,       set_p,
             present, set_present, 0);

    // Create ReadOnly flag 'extended' (and 'e') at position 1.
    flag_ro!(e, extended, 1);

    // Shorter version of a flag_rw! macro.
    flag_rw!(a, set_a, 2);

    // Shorter version of a flag_ro! macro.
    flag_ro!(b, 3);
}

fn some_fn() {
    // ...

    let mut something = Something(0);

    // This means the same thing:
    if (something.p()) { /* ... */ }
    if (something.present()) { /* ... */ }

    // As does this:
    something.set_present(true);
    something.set_p(true);

    // ...
}
```

## Note
This IS NOT a clone of "bitflags" crate from Rust Compiler Developers. It is though my own alternative view of how bitflags in Rust must be and I believe that in some cases this version is more apropriate and easier to use and understand. Furthermore, it does not conflict with 'bitflags'. You can use both and it works okay.
