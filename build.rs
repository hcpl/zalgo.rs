extern crate rustc_version;

use rustc_version::{Version, version};


fn main() {
    let version = version().unwrap();

    if version >= Version::parse("1.26.0").unwrap() {
        println!("cargo:rustc-cfg=stable_fused_iterator");
    }
}
