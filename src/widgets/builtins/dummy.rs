use crate::prelude::{Metadata, Terminal, TerminalConst};
use crate::terminal::{Rectangle, UpdateInfo, UpdateResult};
use crate::widgets::{BoundingBox, Widget};

/// A [`Widget`] that doesn't do anything.
pub struct Dummy;

impl Widget for Dummy {
    fn update(&mut self, _update_info: UpdateInfo, _terminal: impl TerminalConst) -> crate::Result<UpdateResult> {
        Ok(UpdateResult::NoEvent)
    }

    fn draw(&self, _update_info: UpdateInfo, _terminal: impl Terminal) -> crate::Result<UpdateResult> {
        Ok(UpdateResult::NoEvent)
    }
}

impl BoundingBox for Dummy {
    fn bounding_box(&self, _terminal: impl Metadata) -> crate::Result<Rectangle> {
        Ok(Rectangle::of_size((0, 0)))
    }

    fn completely_covers(&self, _rectangle: Rectangle) -> bool {
        true
    }
}