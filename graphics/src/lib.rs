//! A bunch of backend-agnostic types that can be leveraged to build a renderer
//! for [`iced`].
//!
//! [`iced`]: https://github.com/hecrj/iced
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(unused_results)]
#![deny(unsafe_code)]
#![forbid(rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg))]
mod antialiasing;
mod defaults;
mod primitive;
mod renderer;
mod transformation;
mod viewport;
mod widget;

pub mod backend;
pub mod font;
pub mod layer;
pub mod triangle;
pub mod window;

#[doc(no_inline)]
pub use widget::*;

pub use antialiasing::Antialiasing;
pub use backend::Backend;
pub use defaults::Defaults;
pub use layer::Layer;
pub use primitive::Primitive;
pub use renderer::Renderer;
pub use transformation::Transformation;
pub use viewport::Viewport;

pub use iced_native::{
    Background, Font, HorizontalAlignment, Point, Rectangle, Size, Vector,
    VerticalAlignment,
};
