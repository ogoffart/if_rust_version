/*!

[![Build Status](https://travis-ci.org/ogoffart/if_rust_version.svg?branch=master)](https://travis-ci.org/ogoffart/if_rust_version)
[![Crates.io](https://img.shields.io/crates/v/if_rust_version.svg)](https://crates.io/crates/if_rust_version)
[![docs](https://docs.rs/if_rust_version/badge.svg)](https://docs.rs/if_rust_version)

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
# #[macro_use] extern crate if_rust_version;
# fn main() {
# use std::mem;
# use std::ptr;
if_rust_version! { >= 1.36 {
    let mut x = std::mem::MaybeUninit::<u32>::uninit();
    unsafe { x.as_mut_ptr().write(32); }
    let xx = unsafe { x.assume_init() };
} else {
    let mut xx : u32 = unsafe { mem::uninitialized() };
    unsafe { ptr::write(&mut xx as *mut u32, 32); }
}}
assert_eq!(xx, 32);
# }
```
*/
#![cfg_attr(not(test_no_submacro), doc = r#"

The macro can be used to declare items or expression.

It can also be usefull to declare macro for pattern you often use.
For example, if we want to declare functions const from rust 1.31 which
introduced the concept:

```
# #[macro_use] extern crate if_rust_version;
# fn main() {
if_rust_version! { >= 1.31 {
    // just a identity macro that forward the item
    macro_rules! const_fn { ($f:item) => { $f } }
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
# }
```
"#)]
/*!

# Minimum Rust version

The minimum rust version is rust 1.12, because previous versions were
not expanding `tt` correcrtly in `macro_rules!`

This crate has no dependencies, and is `#![no-std]`.

# Comparison with other crates

There are other crates that check the rust version number:
[version_check](https://crates.io/crates/version_check) and
[rustc_version](https://crates.io/crates/rustc_version).

The main difference is that these crates are meant to to be
used by first writing a `build.rs` script to then add some
feature flag.

*/

#![no_std]

include!(concat!(env!("OUT_DIR"), "/generated.rs"));


