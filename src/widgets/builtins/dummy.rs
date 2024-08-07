use crate::Error;
use crate::prelude::{Terminal, TerminalConst};
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
    fn bounding_box(&self, _rect: Rectangle) -> crate::Result<Rectangle> {
        Ok(Rectangle::of_size((0, 0)))
    }

    fn completely_covers(&self, _rectangle: Rectangle) -> bool {
        true
    }
}

/// Want to fill in a widget with a dummy implementation while outlining your code?
///
/// You can do that using this trait. It automatically implements [`Widget`] for you.
#[allow(clippy::module_name_repetitions)]
pub trait EmptyWidget {}

impl<T: EmptyWidget> Widget for T {
    fn update(&mut self, _update_info: UpdateInfo, _terminal: impl TerminalConst) -> crate::Result<UpdateResult> {
        Err(Error::Todo)
    }

    fn draw(&self, _update_info: UpdateInfo, _terminal: impl Terminal) -> crate::Result<UpdateResult> {
        Err(Error::Todo)
    }
}