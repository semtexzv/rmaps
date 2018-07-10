extern crate cc;


pub fn main() {
    cc::Build::new()
        .include("native")
        .file("native/earcut.cpp")
        .compile("earcut.a");
}