//! Turns a resolved model + [`crate::layout`] geometry into the final ANSI
//! string via [`render`].

pub mod renderer;

pub use renderer::render;
