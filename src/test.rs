// Create some structure which will act like a bitfield structure in test.
struct Something(i64);

impl Something {

    // Create ReadWrite flag called 'present' (or shorthand 'p').
    // To set a value use 'set_p' or 'set_present'.
    // Flag position is bit number 0.
    flag_rw!(p, set_p, present, set_present, 0);
}

#[test]
fn some_fn() {
    let mut something = Something(0);

    something.set_present(true);
    assert!(something.present());

    something.set_p(false);
    assert!(!something.p());
}
