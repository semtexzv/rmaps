use ::prelude::*;
use super::*;


pub trait InputHandler {
    fn has_captured(&mut self) -> bool;
    fn mouse_moved(&mut self, pos: PixelPoint) -> bool ;
    fn mouse_button(&mut self, pressed : bool) -> bool ;
    fn mouse_scroll(&mut self, scroll : f64) -> bool;
}