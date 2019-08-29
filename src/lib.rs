

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[macro_export]
/// Yo
macro_rules! if_rust_version {
    (if rust_version $($tail:tt)*) => { if_rust_version!{ $($tail)* } };
    ($op:tt $n:tt { $($if:tt)* }) => { if_rust_version_impl!{ $op $n {$($if)*} {}} };
    ($op:tt $n:tt { $($if:tt)* } else { $($else:tt)* }) => { if_rust_version_impl!{ $op $n {$($if)*} {$($else)*} } };
    ($op:tt $n:tt { $($if:tt)* } else if rust_version $($tail:tt)*) => { if_rust_version_impl!{ $op $n {$($if)*} { if_rust_version!{$($tail)*} } } };
}

