use ::prelude::*;
use super::*;


pub trait InputHandler {
    fn mouse_moved(&mut self, pos: PixelPoint);
    fn mouse_button(&mut self, pressed : bool) -> bool;
    fn mouse_scroll(&mut self, scroll : f64);
}