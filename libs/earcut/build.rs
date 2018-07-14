extern crate cc;


pub fn main() {
    cc::Build::new()
        .include("native")
        .file("native/earcut.cpp")
        .cpp(true)
        .flag("-std=c++14")
        .compile("earcut.a");
}