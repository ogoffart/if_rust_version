/*!
This is a small crate that just export one macro that allow you to have code
conditionally of the rust version.

The macro is still working with Rust 1.0 and allow you to still support old version
of rustc while still using conditionally new features of the compiler

# Examples

The release of rust 1.36 stabilized the `MaybeUninit` union as a replacement
for the to be deprecated `mem::uninitialized`. One might want to use
MaybeUninit when the compiler supports it, but not without requiring a recent compiler.

The if_rust_version! macro is exactly what one needs:

```
#[macro_use]
# use if_rust_version::if_rust_version;
# use std::mem;
if_rust_version! { >= 1.36 {
    let mut x = std::mem::MaybeUninit::<u32>::uninit();
    unsafe { x.as_mut_ptr().write(32); }
    let xx = unsafe { x.assume_init() };
} else {
    let mut xx : u32 = unsafe { mem::uninitialized() };
    unsafe { (&xx as *mut u32).write(32); }
}}
assert_eq!(xx, 32);
```

The macro can be used to declare items or expression.

It can also be usefull to declare macro for pattern you often use.
For example, if we want to declare functions const from rust 1.31 which
introduced the concept:

```
# use if_rust_version::if_rust_version;
if_rust_version! { >= 1.31 {
    // just a identity macro that forward the item
    macro_rules! const_fn { ($fn:item) => { $fn } }
} else {
    // remove the 'const'
    macro_rules! const_fn {
        ($(#[$m:meta])* const fn $($rest:tt)*) => { $(#[$m])* fn $($rest)* };
        ($(#[$m:meta])* pub const fn $($rest:tt)*) => {
            $(#[$m])*
            /// This function is a const fn from rust 1.31
            pub fn $($rest)*
        };
    }
}}

const_fn!{
    /// This function is const chen the compiler supports it
    pub const fn hello(x : u32) -> u32 { x + 2 }
}
```

*/

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[macro_export]
/// Yo
macro_rules! if_rust_version {
    (if rust_version $($tail:tt)*) => { if_rust_version!{ $($tail)* } };
    ($op:tt $n:tt { $($if:tt)* }) => { $crate::if_rust_version_impl!{ $op $n {$($if)*} {}} };
    ($op:tt $n:tt { $($if:tt)* } else { $($else:tt)* }) => { $crate::if_rust_version_impl!{ $op $n {$($if)*} {$($else)*} } };
    ($op:tt $n:tt { $($if:tt)* } else if rust_version $($tail:tt)*) => { $crate::if_rust_version_impl!{ $op $n {$($if)*} { if_rust_version!{$($tail)*} } } };
}

