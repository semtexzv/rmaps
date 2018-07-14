//! Safe rust wrapper around the subset of libtess2 I personally need.
//!
//! Your vertices are ordered clockwise when using the clipping functions.

extern crate tess2_sys as sys;
extern crate itertools;

use sys::*;

use itertools::Itertools;
use std::mem;

pub type Vertex = [sys::TESSreal; 2];

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Triangles {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<sys::TESSindex>,
}

pub struct Tessellator {
    tess: *mut TESStesselator,
}

enum Orientation {
    Clockwise,
    CounterClockwise,
}

impl Tessellator {
    pub fn new() -> Self {
        Self { tess: unsafe { tessNewTess(0 as *mut TESSalloc) } }
    }

    pub fn add_ring(&mut self, ring: &[Vertex]) -> Result<(), String> {
        use std::os::raw::c_void;

        if ring.len() < 3 {
            return Err(String::from("A polygon must have at least 3 vertices."));
        }

        unsafe {
            tessAddContour(self.tess,
                           2,
                           ring.as_ptr() as *const sys::TESSreal as *const c_void,
                           mem::size_of::<Vertex>() as _,
                           ring.len() as _ );
        }


        Ok(())
    }

    pub fn tessellate(&mut self, rule: TessWindingRule) -> Result<Triangles, String> {
        unsafe {
            use std::slice;
            if tessTesselate(self.tess,
                             rule as _,
                             TessElementType_TESS_POLYGONS as _,
                             3,
                             2,
                             0 as *mut TESSreal) != 1 {
                return Err(String::from("Triangulation failed."));
            }

            let raw_triangle_count = tessGetElementCount(self.tess);
            if raw_triangle_count < 1 {
                return Err(String::from("Triangulation failed to yield triangles."));
            };
            let triangle_count = raw_triangle_count as usize;

            let vertex_buffer = slice::from_raw_parts(tessGetVertices(self.tess),
                                                      tessGetVertexCount(self.tess) as usize * 2);
            let triangle_buffer = slice::from_raw_parts(tessGetElements(self.tess),
                                                        triangle_count * 3);

            let xs = vertex_buffer.iter().step(2);
            let ys = vertex_buffer.iter().skip(1).step(2);
            let verts = xs.zip(ys);

            Ok(Triangles {
                vertices: verts.map(|(x, y)| [*x, *y]).collect(),
                indices: triangle_buffer.iter().map(|i| *i as _).collect(),
            })
        }
    }

    pub fn tessellate_nonzero(&mut self) -> Result<Triangles, String> {
        self.tessellate(TessWindingRule_TESS_WINDING_NONZERO)
    }
}

impl Drop for Tessellator {
    fn drop(&mut self) {
        unsafe { tessDeleteTess(self.tess) }
    }
}

pub fn fill(poly: &Vec<Vec<Vertex>>) -> Result<Triangles, String> {
    let mut tess = Tessellator::new();
    for ring in poly.iter() {
        tess.add_ring(&ring[..])?;
    }
    tess.tessellate(TessWindingRule_TESS_WINDING_NONZERO)
}

pub fn intersect(poly1: &Vec<Vec<Vertex>>, poly2 : &Vec<Vec<Vertex>>) -> Result<Triangles, String> {
    let mut tess = Tessellator::new();
    for ring in poly1.iter() {
        tess.add_ring(&ring[..])?;
    }
    for ring in poly2.iter() {
        tess.add_ring(&ring[..])?;
    }
    tess.tessellate(TessWindingRule_TESS_WINDING_ABS_GEQ_TWO)
}
/*
/// Tessellates the union of the given simple polygon paths.
pub fn fill_union<V: VertexLike>(polies: &[&[V]]) -> Result<Triangles, String> {
    let mut tess = Tessellator::new();
    for poly in polies {
        tess.add_poly(poly.iter())?;
    }
    tess.tessellate(TessWindingRule::TESS_WINDING_NONZERO)
}

/// Tessellates the intersection of the given simple polygon paths. To tessellate
/// many, call this function again on the resulting triangles; may become expensive.
pub fn fill_intersection<V: VertexLike>(a: &[V], b: &[V]) -> Result<Triangles, String> {
    let mut tess = Tessellator::new();
    tess.add_poly(a.iter())?;
    tess.add_poly(b.iter())?;
    tess.tessellate(TessWindingRule::TESS_WINDING_ABS_GEQ_TWO)
}

/// Fill tessellate a simple polygon path.
pub fn fill(poly: &[Vertex]) -> Result<Triangles, String> {
    fill_union(&[poly])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(fill(&[Vertex { x: 0.0, y: 0.0 },
            Vertex { x: 1.0, y: 0.0 },
            Vertex { x: 1.0, y: 1.0 },
            Vertex { x: 0.0, y: 1.0 }])
                       .expect("triangulation"),
                   Triangles {
                       vertices: vec![Vertex { x: 0.0, y: 1.0 },
                                      Vertex { x: 1.0, y: 0.0 },
                                      Vertex { x: 1.0, y: 1.0 },
                                      Vertex { x: 0.0, y: 0.0 }],
                       indices: vec![0, 1, 2, 1, 0, 3],
                   });
    }

    #[test]
    fn intersection() {
        assert_eq!(fill_intersection(&[Vertex { x: 0.0, y: 0.0 },
            Vertex { x: 1.0, y: 0.0 },
            Vertex { x: 1.0, y: 1.0 },
            Vertex { x: 0.0, y: 1.0 }],
                                     &[Vertex { x: 0.25, y: 0.25 },
                                         Vertex { x: 0.75, y: 0.25 },
                                         Vertex { x: 0.75, y: 0.75 },
                                         Vertex { x: 0.25, y: 0.75 }])
                       .expect("triangulation"),
                   Triangles {
                       vertices: vec![Vertex { x: 0.25, y: 0.75 },
                                      Vertex { x: 0.75, y: 0.25 },
                                      Vertex { x: 0.75, y: 0.75 },
                                      Vertex { x: 0.25, y: 0.25 }],
                       indices: vec![0, 1, 2, 1, 0, 3],
                   });
    }

    #[test]
    fn union() {
        assert_eq!(fill_union(&[&[Vertex { x: 0.0, y: 0.0 },
            Vertex { x: 2.0, y: 4.0 },
            Vertex { x: 4.0, y: 0.0 }],
            &[Vertex { x: 0.5, y: 0.0 },
                Vertex { x: 2.0, y: 2.0 },
                Vertex { x: 3.5, y: 0.0 }]])
                       .expect("triangulation"),
                   Triangles {
                       vertices: vec![Vertex { x: 2.0, y: 2.0 },
                                      Vertex { x: 4.0, y: 0.0 },
                                      Vertex { x: 3.5, y: 0.0 },
                                      Vertex { x: 2.0, y: 4.0 },
                                      Vertex { x: 0.5, y: 0.0 },
                                      Vertex { x: 0.0, y: 0.0 }],
                       indices: vec![0, 1, 2, 0, 3, 1, 4, 3, 0, 3, 4, 5, 2, 4, 0],
                   });
    }

    #[test]
    fn difference() {
        assert_eq!(fill_difference(&[Vertex { x: 0.0, y: 0.0 },
            Vertex { x: 2.0, y: 4.0 },
            Vertex { x: 4.0, y: 0.0 }],
                                   &[&[Vertex { x: 0.5, y: 0.0 },
                                       Vertex { x: 2.0, y: 2.0 },
                                       Vertex { x: 3.5, y: 0.0 }]])
                       .expect("triangulation"),
                   Triangles {
                       vertices: vec![Vertex { x: 2.0, y: 2.0 },
                                      Vertex { x: 4.0, y: 0.0 },
                                      Vertex { x: 3.5, y: 0.0 },
                                      Vertex { x: 2.0, y: 4.0 },
                                      Vertex { x: 0.5, y: 0.0 },
                                      Vertex { x: 0.0, y: 0.0 }],
                       indices: vec![0, 1, 2, 0, 3, 1, 4, 3, 0, 3, 4, 5],
                   });
    }
}
*/