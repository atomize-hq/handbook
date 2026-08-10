mod error;
#[cfg(all(test, unix))]
mod inspect;
mod json;
#[cfg(test)]
mod markdown;
mod model;
mod shared;

pub(crate) use json::render_json;
pub(crate) use model::{build_output_model, RenderOutputModel};

#[cfg(test)]
pub(crate) use error::RenderError;
#[cfg(all(test, unix))]
pub(crate) use inspect::render_inspect;
#[cfg(test)]
pub(crate) use markdown::render_markdown;
