pub use ::common::export::*;
pub use common::failure;

pub use map::pal;

pub use std::convert::{TryFrom, From, Into};
pub use rmaps_derive::*;


use common::actix::{ResponseFuture, ResponseActFuture};



use common::glium::uniforms::{Uniforms, UniformValue};

pub struct MergeUniforms<'u, A: Uniforms + 'u, B: Uniforms + 'u> (pub &'u A, pub &'u B);

pub fn merge_uniforms<'u, A: Uniforms + 'u, B: Uniforms + 'u>(a: &'u A, b: &'u B) -> MergeUniforms<'u, A, B> {
    MergeUniforms(a, b)
}

impl<'u, A: Uniforms + 'u, B: Uniforms + 'u> Uniforms for MergeUniforms<'u, A, B> {
    fn visit_values<'a, F: FnMut(&str, UniformValue<'a>)>(&'a self, mut f: F) {
        self.0.visit_values(&mut f);
        self.1.visit_values(f);
    }
}



use std::convert::AsMut;

pub fn make_slice<A, T>(slice: &[T]) -> A
    where A: Sized + Default + AsMut<[T]>,
          T: Clone
{
    let mut a = Default::default();
    <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
    a
}