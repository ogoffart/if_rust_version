#![allow(unknown_lints)]
#![allow(bare_trait_objects)]

/// Parse the output of rustc --version and get the minor version and the channel
fn parse_rustc_version(version_output: &[u8]) -> (u32, &str) {
    let version_output = ::std::str::from_utf8(version_output).expect("Cannot parse rustc version");

    let version_string = version_output
        .split_whitespace()
        .nth(1)
        .expect("Cannot parse rustc version");

    let mut dashsplit = version_string.split('-');
    let mut ver_iter = dashsplit.next().expect("Invalid version number").split('.');
    assert_eq!(ver_iter.next(), Some("1"), "Only Rust 1.x is supported");

    let ver_minor: u32 = ver_iter
        .next()
        .and_then(|x: &str| x.parse::<u32>().ok())
        .expect("Cannot parse rustc version number");

    (ver_minor, dashsplit.next().unwrap_or(""))
}

#[test]
fn parse_rustc_version_test() {
    assert_eq!(parse_rustc_version(b"rustc 1.37"), (37, ""));
    assert_eq!(parse_rustc_version(b"rustc 1.37\n"), (37, ""));
    assert_eq!(parse_rustc_version(b"rustc 1.37.0"), (37, ""));
    assert_eq!(parse_rustc_version(b"rustc 1.37.1"), (37, ""));
    assert_eq!(parse_rustc_version(b"rustc 1.37.1\n"), (37, ""));
    assert_eq!(parse_rustc_version(b"rustc 1.0"), (0, ""));
    assert_eq!(parse_rustc_version(b"rustc 1.10 (something)"), (10, ""));
    assert_eq!(
        parse_rustc_version(b"rustc 1.88.0-nightly"),
        (88, "nightly")
    );
    assert_eq!(
        parse_rustc_version(b"rustc 1.13-nightly\n"),
        (13, "nightly")
    );
    assert_eq!(
        parse_rustc_version(b"rustc 1.37.0-nightly (d132f544f 2019-06-07)"),
        (37, "nightly")
    );
}

fn generate<T: ::std::io::Write>(mut f: T, ver_minor: u32, channel: &str) {
    let crate_ = if ver_minor >= 30 { "$crate::" } else { "" };

    writeln!(&mut f, "#[doc(hidden)] #[macro_export]").unwrap();
    writeln!(&mut f, "macro_rules! if_rust_version_impl {{").unwrap();
    for ver in 0..(ver_minor + 1) {
        writeln!(&mut f, "    (>= 1.{} {{ $($if_:tt)* }} {{ $($else_:tt)* }}) => {{ $($if_)* }};", ver).unwrap();
    }
    writeln!(&mut f, "    (>= $n:tt {{ $($if_:tt)* }} {{ $($else_:tt)* }} ) => {{ $($else_)* }};").unwrap();
    writeln!(&mut f, "    (== 1.{} {{ $($if_:tt)* }} {{ $($else_:tt)* }}) => {{ $($if_)* }};", ver_minor).unwrap();
    writeln!(&mut f, "    (== nightly {{ $($if_:tt)* }} {{ $($else_:tt)* }}) => {{ $(${})* }};",
        if channel == "nightly" { "if_" } else { "else_" }).unwrap();
    writeln!(&mut f, "    (== $n:tt {{ $($if_:tt)* }} {{ $($else_:tt)* }}) => {{ $($else_)* }};").unwrap();
    writeln!(&mut f, "    (> 1.{} {{ $($if_:tt)* }} {{ $($else_:tt)* }}) => {{ $($else_)* }};", ver_minor).unwrap();
    writeln!(&mut f, "    (> $n:tt {{ $($if_:tt)* }} {{ $($else_:tt)* }}) => {{ {}if_rust_version_impl!{{>= $n {{$($if_)*}} {{$($else_)*}} }} }};", crate_).unwrap();
    writeln!(&mut f, "    (!= $n:tt {{ $($if_:tt)* }} {{ $($else_:tt)* }}) => {{ {}if_rust_version_impl!{{== $n {{$($else_)*}} {{$($if_)*}} }} }};", crate_).unwrap();
    writeln!(&mut f, "    (< $n:tt {{ $($if_:tt)* }} {{ $($else_:tt)* }}) => {{ {}if_rust_version_impl!{{ >= $n {{$($else_)*}} {{$($if_)*}} }} }};", crate_).unwrap();
    writeln!(&mut f, "    (<= $n:tt {{ $($if_:tt)* }} {{ $($else_:tt)* }}) => {{ {}if_rust_version_impl!{{ > $n {{$($else_)*}} {{$($if_)*}} }} }};", crate_).unwrap();
    writeln!(&mut f, "}}").unwrap();

    let doc = if ver_minor < 30 {
        ""
    } else {
        r#"/**
This macro can enable or disable code depending on the rust version with which the program is
compiled.

The syntax is this:
```text
(if rust_version)? <operator> <version> { <code> } (else if rust_version <operator> <version> { <code> })* (else { <code> })?
```

So first a block for

The operator is one of `==`, `!=`, `>=`, `<=`, `<` or `>`. The version is either `nightly` or a
version number in the form `1.x`.

**Important:** The version number can only have one period, and start with `1.`. So for example
simply `1.36`, **but NOT** `1.36.0` or `0.42`

The macro will expand to the code corresponding to the right condition. (Or nothing if no
condition match).

Examples:

```rust
# use if_rust_version::if_rust_version;
if_rust_version!{ == nightly {
    fn foo() { /* implementation on nightly */ }
} else if rust_version >= 1.33 {
    fn foo() { /* implementation on rust 1.33 or later */ }
} else {
    fn foo() { /* implementation on rust 1.33 on old rust */ }
}}
```

```rust
# use if_rust_version::if_rust_version;
if_rust_version!{ >= 1.36 { use std::mem::MaybeUninit; }}
// ...
if_rust_version!{ < 1.36 {
    let mut foo: u32 = unsafe { ::std::mem::uninitialized() };
} else {
    let mut foo: u32 = unsafe { ::std::mem::MaybeUninit::uninit().assume_init() };
}}
```

Note that in case this is used as an expression, no blocks will be added.

```error
// Error
println!("{}", if_rust_version!{ < 1.22 { let x = 42; x} else { 43 } } );
```

```rust
# use if_rust_version::if_rust_version;
// ok
println!("{}", { if_rust_version!{ < 1.22 { let x = 42; x} else { 43 } } } );
// Also ok
println!("{}", if_rust_version!{ < 1.22 { {let x = 42; x} } else { 43 } }  );
```

*/"#
    };

    writeln!(&mut f, r#"
{doc}
#[macro_export]
macro_rules! if_rust_version {{
    (if rust_version $($tail:tt)*) => {{ if_rust_version!{{ $($tail)* }} }};
    ($op:tt $n:tt {{ $($if_:tt)* }}) => {{ {crate}if_rust_version_impl!{{ $op $n {{$($if_)*}} {{}}}} }};
    ($op:tt $n:tt {{ $($if_:tt)* }} else {{ $($else_:tt)* }}) => {{ {crate}if_rust_version_impl!{{ $op $n {{$($if_)*}} {{$($else_)*}} }} }};
    ($op:tt $n:tt {{ $($if_:tt)* }} else if rust_version $($tail:tt)*) => {{ {crate}if_rust_version_impl!{{ $op $n {{$($if_)*}} {{ if_rust_version!{{$($tail)*}} }} }} }};
}}"#, crate = crate_, doc = doc).unwrap();
}

fn main() {
    let version_output = ::std::process::Command::new(
        ::std::env::var_os("RUSTC")
            .as_ref()
            .map(|x| x as &AsRef<::std::ffi::OsStr>)
            .unwrap_or(&"rustc"),
    )
    .arg("--version")
    .output()
    .expect("Could not run rustc")
    .stdout;

    let (ver_minor, channel) = parse_rustc_version(&version_output);

    let f = ::std::fs::File::create(
        &::std::path::Path::new(&::std::env::var_os("OUT_DIR").unwrap()).join("generated.rs"),
    )
    .unwrap();
    generate(f, ver_minor, channel);

    if ver_minor < 14 {
        println!("cargo:rustc-cfg=test_no_submacro");
    }
}
