extern crate cc;


fn main() {
    cc::Build::new().include("native/include/")
        .file("native/src/bucketalloc.c")
        .file("native/src/dict.c")
        .file("native/src/geom.c")
        .file("native/src/mesh.c")
        .file("native/src/priorityq.c")
        .file("native/src/sweep.c")
        .file("native/src/tess.c")
        .pic(true)
        .compile("libtess2.a");
}