pub mod callback;
pub mod components;
mod elem;
mod interfaces;
mod shared;

pub mod prelude {
    pub use crate::callback::Callback;
    pub use crate::components::*;
    pub use crate::elem::{html_props, HtmlProps};
}
