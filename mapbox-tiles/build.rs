extern crate prost_build;

fn main() {
    prost_build::compile_protos(&["proto/vector_tile.proto"],
                                &["proto/"]).unwrap();
}