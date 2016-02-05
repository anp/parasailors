// Copyright (c) 2016 Adam Perry <adam.n.perry@gmail.com>
//
// This software may be modified and distributed under the terms of the MIT license.  See the
// LICENSE file for details.

extern crate cmake;
use cmake::Config;

fn main() {
    let dst = Config::new("parasail_c").define("CMAKE_POSITION_INDEPENDENT_CODE", "TRUE").build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=parasail");
}
