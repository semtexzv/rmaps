pub mod ffi;
pub use ffi::*;

#[test]
fn test_link(){
    unsafe {
        let a = tessNewTess(0 as *mut _);
        tessDeleteTess(a);
    }
}

#[test]
fn test_basic_tess(){
    let mut data = [
        0.0,0.0,
       // 0.3,0.3f32,
        1.0,0.0f32,
        1.0,1.0,
        0.0,1.0,
    ];unsafe {
        let nvp = 3usize;
        let fpv = 2;
        let bpv = 2 * std::mem::size_of::<f32>();
        let mut tess = tessNewTess(0 as *mut _);
        tessAddContour(tess,2,data.as_ptr() as _, 2 * std::mem::size_of::<f32>() as i32,(data.len()/2) as _);

       /* tessAddContour(tess,2,data.as_ptr() as _, 2 * std::mem::size_of::<f32>() as i32,(data.len()/2) as _); data[0] = -1.0;
        data[1] = -1.0;

        tessAddContour(tess,2,data.as_ptr() as _, 2 * std::mem::size_of::<f32>() as i32,(data.len()/2) as _);
        */
      //  tessAddContour(tess,2,data.as_ptr() as _,0,(data.len()/2) as _);

        println!("STATUS:{:?}", tessTesselate(tess,TessWindingRule::TESS_WINDING_POSITIVE,TessElementType::TESS_POLYGONS,nvp as i32,2,0 as *const _));
        println!("Elems:{:?} {:?}",tessGetElements(tess),tessGetElementCount(tess));
        println!("Elems:{:?} {:?}",tessGetVertices(tess),tessGetVertexCount(tess));
        println!("Elems:{:?} {:?}",tessGetVertexIndices(tess),tessGetVertexCount(tess));
        println!("Elems:{:?}",std::slice::from_raw_parts(tessGetElements(tess),((tessGetElementCount(tess) as usize)*nvp)));
        println!("Verts:{:?}",std::slice::from_raw_parts(tessGetVertices(tess) as *const f32,tessGetVertexCount(tess) as usize * fpv));
        println!("Indis:{:?}",std::slice::from_raw_parts(tessGetVertexIndices(tess),tessGetVertexCount(tess) as _));


    }
}