use web_sys::{window, HtmlDivElement};

use crate::{
    component::ComponentBuilder,
    hooks::{use_element, use_ref, ReiaRef},
    ContainerReturn,
};
use wasm_bindgen::JsCast;

#[derive(PartialEq, Clone)]
pub struct DivProps {}

pub fn div(reia: ComponentBuilder, _props: DivProps) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.parent_node.clone();
    let (reia, div): (_, HtmlDivElement) = reia.hook(use_element, ("div", parent));
    reia.bare_container_node(div.into())
}
