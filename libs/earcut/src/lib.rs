#![allow(dead_code, non_upper_case_globals, non_camel_case_types)]

mod ffi;

use ffi::*;


pub fn earcut(feature: &Vec<Vec<[::ffi::COORD_TYPE; 2]>>) -> Vec<::ffi::INDEX_TYPE> {
    use std::mem;
    use std::slice;

    unsafe {
        let e = ffi::earcut_new();
        // No empty rings
        assert!(feature.iter().all(|r| !r.is_empty()));
        for r in feature.iter() {
            ffi::earcut_ring(e, mem::transmute(r.as_ptr()), (r.len() * 2) as _);
        }

        ffi::earcut_tesselate(e);

        let borrowed = slice::from_raw_parts(ffi::earcut_data(e), ffi::earcut_size(e));


        let res = Vec::from(borrowed);

        ffi::earcut_delete(e);
        res
    }
}