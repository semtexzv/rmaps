pub mod resource;
pub mod response;

pub use self::resource::*;


pub trait Source {
    // TODO, async loading, Callbacks or messages,
    // Resource -> Response
}