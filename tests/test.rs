#[macro_use]
extern crate if_rust_version;

#[test]
fn basic() {
    let foo = if_rust_version! { >= 1.0 { "Hello" } else { "World" ;; error ;; }};
    assert_eq!(foo, "Hello");

    let fo2 = if_rust_version! { >= 1.999 { "Hello" ;; error ;; } else { "World" }};
    assert_eq!(fo2, "World");

    if_rust_version! { < 1.0 { let x = :error:; } else { let x = 24; }};
    assert_eq!(x, 24);

    if_rust_version! { < 1.999 { let x = 44; } else { let x = ~error~; }};
    assert_eq!(x, 44);
}

if_rust_version! { >= 1.0 { fn fn1() -> bool { true } } else { something wrong } }
if_rust_version! { <= 1.999 { fn fn2() -> bool { true } } }
if_rust_version! { != 1.999 { fn fn3() -> bool { true } } else {} }
if_rust_version! { < 1.0 {  } else if rust_version >= 1.0 { fn fn4() -> bool { true } } }
if_rust_version! { > 1.999 { something wrong } else if rust_version != 1.998 { fn fn5() -> bool { true } } else { } }

#[test]
fn testfn() {
    assert!(fn1());
    assert!(fn2());
    assert!(fn3());
    assert!(fn4());
    assert!(fn5());
}

#[test]
fn test_uninit() {
    unsafe fn init_bool(x: *mut bool) {
        std::ptr::write(x, true);
    }

    let b: bool = {
        if_rust_version! { >= 1.36 {
            let mut x = std::mem::MaybeUninit::<bool>::uninit();
            unsafe { init_bool(x.as_mut_ptr()); }
            unsafe { x.assume_init() }
        } else {
            let mut x : bool = unsafe { std::mem::uninitialized() };
            unsafe { init_bool(&mut x as *mut bool) }
            x
        }}
    };
    assert_eq!(b, true);
}

if_rust_version! { == 1.37 {
    #[test]
    fn test_37() {
        if_rust_version!(== nightly { error });
        assert!(if_rust_version!(== nightly { ~ error ~ } else { true }));
        assert!(if_rust_version!(== nightly { ~ error ~ } else if rust_version != nightly { true }));
        assert!(if_rust_version!(== nightly { ~ error ~ } else if rust_version != nightly { true } else { ~error~ }));

        assert!(if_rust_version!(> 1.37 { ~ error ~ } else if rust_version > 1.36 { true } else { ~error~ }));
        assert!(if_rust_version!(< 1.37 { ~ error ~ } else if rust_version < 1.38 { true } else { ~error~ }));
        assert!(if_rust_version!(< 1.34 { ~ error ~ } else if rust_version <= 1.37 { true } else { ~error~ }));
        assert!(if_rust_version!(< 1.32 { ~ error ~ } else if rust_version <= 1.36 { error~ } else { true }));
        assert!(if_rust_version!(!= 1.34 { true }));
        assert!(if_rust_version!(!= 1.36 { true }));
        assert!(if_rust_version!(!= 1.38 { true }));
        if_rust_version!(== 1.36 { ~ error } else if rust_version == 1.38 { ~ error });
        if_rust_version!(<= 1.36 { ~ error } else if rust_version >= 1.38 { ~ error });
        if_rust_version!(< 1.36 { ~ error } else if rust_version > 1.38 { ~ error });
        if_rust_version!(< 1.37 { ~ error } else if rust_version > 1.37 { ~ error });
        if_rust_version!(!= 1.37 { ~ error } else if rust_version > 1.39 { ~ error });
    }
}}

#[test]
fn more_tests() {
    if_rust_version!(== nightly { let x = 2; } else if rust_version != nightly { let x = 3; });
    assert!(x != 1);

    if_rust_version!(> 1.31 { fn a() -> u32 { 1 } });
    if_rust_version!(<= 1.31 { fn a() -> u32 { 1 } });

    if_rust_version!(== 1.10 { fn b() -> u32 { 1 } });
    if_rust_version!(!= 1.10 { fn b() -> u32 { 1 } });

    if_rust_version!(> 1.0 { fn c() -> u32 { 1 } });
    if_rust_version!(== 1.0 { fn c() -> u32 { 1 } });

    if_rust_version!(< 1.13 { fn d() -> u32 { 1 } });
    if_rust_version!(>= 1.13 { fn d() -> u32 { 1 } });

    if_rust_version!(< 1.28 { fn e() -> u32 { 1 } });
    if_rust_version!(== 1.28 { fn e() -> u32 { 1 } });
    if_rust_version!(> 1.28 { fn e() -> u32 { 1 } });

    assert_eq!(a() + b() + c() + d() + e(), 5);
}

#[test]
fn item_expr() {
    assert_eq!(
        {
            if_rust_version! { > 1.2 {
                fn foo_1() -> u32 { 11 }
                foo_1()
            } else {
                fn foo_2() -> u32 { 11 }
                foo_2()
            }}
        },
        11
    );
}

#[cfg(not(test_no_submacro))]
mod xx {

    if_rust_version! { < 1.31 {
        macro_rules! const_fn {
            ($(#[$m:meta])* const fn $($rest:tt)*) => {
                $(#[$m])* fn $($rest)*
            };
            ($(#[$m:meta])* pub const fn $($rest:tt)*) => {
                $(#[$m])*
                ///
                /// This function is a const fn from rust 1.33
                pub fn $($rest)*
            };
        }
    } else {
        macro_rules! const_fn { ($f:item) => { $f } }
    }}

    const_fn! {
        /// Function which is constant for some version of the compiler
        #[inline]
        pub const fn foo_const(x : u32) -> u32 { x + 2 }
    }

    #[test]
    fn test_const_fn() {
        assert_eq!(foo_const(44), 46);
    }
}

#[test]
fn new_syntax() {
    let x = if_rust_version!(>= 1.26 { 1000u128 } else { 1000 });
    assert_eq!(x, 1000);
    if_rust_version! { < 1.999 { let x = 1010; } else { let x = 1010u543; }};
    assert_eq!(x, 1010);
}
