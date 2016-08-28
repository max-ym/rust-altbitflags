# alt_bitflags
If you need to create a lot of many-many-many bitfields in an intuitive way, check out some of these macro.

It is not a C-like bitfield structure generator. It's more like a named bitmap. Still, if you need to change often single bits in integer numbers you may access them by functions that are generated with these macros.

## Demo
```rust
// Make sure to write #[macro_use]!
#[macro_use]
extern crate alt_bitflags;

// Create some structure which will act like a bitfield structure.
struct Something(u32);

impl Something {

    // Create ReadWrite flag called 'present' (or shorthand 'p').
    // To set a value use 'set_p' or 'set_present'.
    // Flag position is bit number 0.
    flag_rw!(p, present, set_p, set_present, 0);

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
    if (something.p) { /* ... */ }
    if (something.present) { /* ... */ }

    // As does this:
    something.set_present(true);
    something.set_p(true);

    // ...
}
```

## Note
This IS NOT a clone of "rust bitflags" crate. It is though my own alternative view of how bitflags in rust must be and I believe that in some cases this version is more apropriate and easier to use and understand. Furthermore, it does not conflict with bitflags from rust compiler. You can use both and it must be okay.

This project still is under heavy developement and was not tested properly! Please, if you're interested, come back later. It'll be ready soon.
