use web_sys::HtmlDivElement;

use crate::use_element::use_element;
use reia::{ComponentBuilder, ContainerReturn};

#[derive(PartialEq, Clone)]
pub struct DivProps {}

pub fn div(reia: ComponentBuilder, _props: DivProps) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, div): (_, HtmlDivElement) = reia.hook(use_element, ("div", parent));
    reia.bare_container_node(div.into())
}
