#![deny(unsafe_code)]

mod island;

pub use island::DefaultIsland;

use protocol::{Island, IslandInit};

/// Builds one concrete Island and hands it to the Chart behind a trait object, so the Chart
/// works with any type that implements `Island`.
/// `Send` is added here because the shared `Island` trait does not require it,
/// while the Chart must be `Send`.
pub type IslandFactory = fn(IslandInit) -> Box<dyn Island + Send>;
