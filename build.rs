#![allow(unknown_lints)]
#![allow(bare_trait_objects)]

/// Parse the output of rustc --version and get the minor version and the channel
fn parse_rustc_version(version_output: &[u8]) -> (u32, &str) {
    let version_output =
        ::std::str::from_utf8(version_output).expect("Cannot parse rustc version");

    let version_string = version_output
        .split_whitespace()
        .nth(1)
        .expect("Cannot parse rustc version");

    let mut dashsplit = version_string.split('-');
    let mut ver_iter = dashsplit.next().expect("Invalid version number").split('.');
    assert_eq!(ver_iter.next(), Some("1"), "Only Rust 1.x is supported");

    let ver_minor: u32 = ver_iter
        .next()
        .and_then(|x : &str| x.parse::<u32>().ok())
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
    writeln!(&mut f, "    (> $n:tt {{ $($if_:tt)* }} {{ $($else_:tt)* }}) => {{ if_rust_version_impl!{{>= $n {{$($if_)*}} {{$($else_)*}} }} }};").unwrap();
    writeln!(&mut f, "    (!= $n:tt {{ $($if_:tt)* }} {{ $($else_:tt)* }}) => {{ if_rust_version_impl!{{== $n {{$($else_)*}} {{$($if_)*}} }} }};").unwrap();
    writeln!(&mut f, "    (< $n:tt {{ $($if_:tt)* }} {{ $($else_:tt)* }}) => {{ if_rust_version_impl!{{ >= $n {{$($else_)*}} {{$($if_)*}} }} }};").unwrap();
    writeln!(&mut f, "    (<= $n:tt {{ $($if_:tt)* }} {{ $($else_:tt)* }}) => {{ if_rust_version_impl!{{ > $n {{$($else_)*}} {{$($if_)*}} }} }};").unwrap();
    writeln!(&mut f, "}}").unwrap();

    writeln!(&mut f, r#"
#[macro_export]
macro_rules! if_rust_version {{
    (if rust_version $($tail:tt)*) => {{ if_rust_version!{{ $($tail)* }} }};
    ($op:tt $n:tt {{ $($if_:tt)* }}) => {{ {crate}if_rust_version_impl!{{ $op $n {{$($if_)*}} {{}}}} }};
    ($op:tt $n:tt {{ $($if_:tt)* }} else {{ $($else_:tt)* }}) => {{ {crate}if_rust_version_impl!{{ $op $n {{$($if_)*}} {{$($else_)*}} }} }};
    ($op:tt $n:tt {{ $($if_:tt)* }} else if rust_version $($tail:tt)*) => {{ {crate}if_rust_version_impl!{{ $op $n {{$($if_)*}} {{ if_rust_version!{{$($tail)*}} }} }} }};
}}"#, crate = if ver_minor >= 30 { "$crate::" } else { "" } ).unwrap();

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
