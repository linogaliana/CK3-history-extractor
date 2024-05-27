/// A submodule handling game localization.
mod localizer;
pub use localizer::Localizer;

/// A submodule that provides [Renderable] and [Cullable] traits for objects that can be rendered.
mod renderer;
pub use renderer::{Renderer, Renderable, Cullable};

/// Map handling submodule.
mod map;
pub use map::GameMap;

/// The graphing submodule that handles the creation of graphs from the game state.
mod graph;
pub use graph::Grapher;

/// A submodule handling the rendering of the timeline page
mod timeline;
pub use timeline::Timeline;
