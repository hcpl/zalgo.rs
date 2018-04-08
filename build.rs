extern crate rustc_version;
#[cfg(feature = "skeptic")]
extern crate skeptic;

use rustc_version::{Version, version};


fn run_rustc_version() {
    let version = version().unwrap();

    if version >= Version::parse("1.26.0").unwrap() {
        println!("cargo:rustc-cfg=stable_fused_iterator");
    }

    if version >= Version::parse("1.27.0").unwrap() {
        // We won't be able to define `Iterator::try_fold` and `DoubleEndedIterator::try_fold`
        // in stable anyway because both require a type parameter bounded by `ops::Try` which is
        // not going to stabilize yet.
        //println!("cargo:rustc-cfg=stable_iterator_try_fold");
        println!("cargo:rustc-cfg=stable_iter_rfold");
    }
}

#[cfg(feature = "skeptic")]
fn run_skeptic() {
    skeptic::generate_doc_tests(&["README.md"]);
}


fn main() {
    run_rustc_version();
    #[cfg(feature = "skeptic")]
    run_skeptic();
}
