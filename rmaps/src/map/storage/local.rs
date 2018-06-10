use prelude::*;

pub struct LocalFileSource {

}


impl LocalFileSource {
    pub fn new() -> LocalFileSource {
        LocalFileSource{

        }
    }
}

impl super::Source for LocalFileSource{
    fn can_handle(&self, url: &str) -> bool {
        return url.starts_with("local://") || url.starts_with("file://");
    }
}