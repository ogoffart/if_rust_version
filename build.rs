#![allow(bare_trait_objects)]

/// Parse the output of rustc --version and get the minor version and the channel
fn parse_rustc_version(version_output: &[u8]) -> (u32, &str) {
    let version_output =
        std::str::from_utf8(version_output).expect("invalid utf-8 in rustc version");

    let version_string = version_output
        .split_whitespace()
        .nth(1)
        .expect("Could not parse rustc version output");

    let mut dashsplit = version_string.split('-');
    let mut ver_iter = dashsplit.next().expect("Invalid version number").split('.');
    assert_eq!(ver_iter.next(), Some("1"), "Only Rust 1.x is supported");

    let ver_minor: u32 = ver_iter
        .next()
        .and_then(|x| x.parse().ok())
        .expect("Could not parse rustc version number");

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

fn generate<T: std::io::Write>(mut f: T, ver_minor: u32, channel: &str) {
    /* writeln!(&mut f, "#[doc(hidden)] #[macro_export]").unwrap();
    writeln!(&mut f, "macro_rules! if_rust_version_impl {{").unwrap();
    for ver in 0..=ver_minor {
        writeln!(&mut f, "    (>= 1.{} $if:block $else:block) => {{ $if }};", ver).unwrap();
    }
    writeln!(&mut f, "    (>= $n:tt $if:block $else:block) => {{ $else }};").unwrap();
    writeln!(&mut f, "    (== 1.{} $if:block $else:block) => {{ $if }};", ver_minor).unwrap();
    writeln!(&mut f, "    (== nightly $if:block $else:block) => {{ ${} }};",
        if channel == "nightly" { "if" } else { "else" }).unwrap();
    writeln!(&mut f, "    (== $n:tt $if:block $else:block) => {{ $else }};").unwrap();
    writeln!(&mut f, "    (> {} $if:block $else:block) => {{ $else }};", ver_minor).unwrap();
    writeln!(&mut f, "    (> $n:tt $if:block $else:block) => {{ if_rust_version_impl!{{>= $n $if $else}} }};").unwrap();
    writeln!(&mut f, "    (!= $n:tt $if:block $else:block) => {{ == $n $else $if }};").unwrap();
    writeln!(&mut f, "    (< $n:tt $if:block $else:block) => {{ if_rust_version_impl!{{ >= $n $else $if}} }};").unwrap();
    writeln!(&mut f, "    (<= $n:tt $if:block $else:block) => {{ if_rust_version_impl!{{ > $n $else $if}} }};").unwrap();
    writeln!(&mut f, "}}").unwrap(); */

    writeln!(&mut f, "#[doc(hidden)] #[macro_export]").unwrap();
    writeln!(&mut f, "macro_rules! if_rust_version_impl {{").unwrap();
    for ver in 0..=ver_minor {
        writeln!(&mut f, "    (>= 1.{} {{ $($if:tt)* }} {{ $($else:tt)* }}) => {{ $($if)* }};", ver).unwrap();
    }
    writeln!(&mut f, "    (>= $n:tt {{ $($if:tt)* }} {{ $($else:tt)* }} ) => {{ $($else)* }};").unwrap();
    writeln!(&mut f, "    (== 1.{} {{ $($if:tt)* }} {{ $($else:tt)* }}) => {{ $($if)* }};", ver_minor).unwrap();
    writeln!(&mut f, "    (== nightly {{ $($if:tt)* }} {{ $($else:tt)* }}) => {{ $(${})* }};",
        if channel == "nightly" { "if" } else { "else" }).unwrap();
    writeln!(&mut f, "    (== $n:tt {{ $($if:tt)* }} {{ $($else:tt)* }}) => {{ $($else)* }};").unwrap();
    writeln!(&mut f, "    (> 1.{} {{ $($if:tt)* }} {{ $($else:tt)* }}) => {{ $($else)* }};", ver_minor).unwrap();
    writeln!(&mut f, "    (> $n:tt {{ $($if:tt)* }} {{ $($else:tt)* }}) => {{ if_rust_version_impl!{{>= $n {{$($if)*}} {{$($else)*}} }} }};").unwrap();
    writeln!(&mut f, "    (!= $n:tt {{ $($if:tt)* }} {{ $($else:tt)* }}) => {{ if_rust_version_impl!{{== $n {{$($else)*}} {{$($if)*}} }} }};").unwrap();
    writeln!(&mut f, "    (< $n:tt {{ $($if:tt)* }} {{ $($else:tt)* }}) => {{ if_rust_version_impl!{{ >= $n {{$($else)*}} {{$($if)*}} }} }};").unwrap();
    writeln!(&mut f, "    (<= $n:tt {{ $($if:tt)* }} {{ $($else:tt)* }}) => {{ if_rust_version_impl!{{ > $n {{$($else)*}} {{$($if)*}} }} }};").unwrap();
    writeln!(&mut f, "}}").unwrap();
}

fn main() {
    let version_output = std::process::Command::new(
        std::env::var_os("RUSTC")
            .as_ref()
            .map(|x| x as &AsRef<std::ffi::OsStr>)
            .unwrap_or(&"rustc"),
    )
    .arg("--version")
    .output()
    .expect("Could not run rustc --version")
    .stdout;

    let (ver_minor, channel) = parse_rustc_version(&version_output);

    let f = std::fs::File::create(
        &std::path::Path::new(&std::env::var_os("OUT_DIR").unwrap()).join("generated.rs"),
    )
    .expect("Could not create the generated.rs");
    generate(f, ver_minor, channel)
}
