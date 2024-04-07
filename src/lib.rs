#![doc = include_str!("../README.md")]

#[macro_use]
extern crate log;

mod ass_writer;
mod canvas;
mod danmu;
mod drawable;
mod model;

pub use ass_writer::AssWriter;
pub use canvas::{Canvas, Config as CanvasConfig};
pub use danmu::Danmu;
pub use drawable::{DrawEffect, Drawable};
pub use model::{DanmakuElem, DmSegMobileReply};
