//! Configuration of our layouts and custom layout algorithms
use penrose::{
    core::layout::{Layout, LayoutStack, Message},
    pure::{geometry::Rect, Stack},
    Xid,
};

pub fn layouts() -> LayoutStack {
    LayoutStack::default()
}
