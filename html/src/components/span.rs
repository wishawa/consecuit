use web_sys::HtmlSpanElement;

use crate::use_element::use_element;
use reia::{ComponentBuilder, ContainerReturn};

#[derive(PartialEq, Clone)]
pub struct SpanProps {}
pub fn span(reia: ComponentBuilder, _props: SpanProps) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, span): (_, HtmlSpanElement) = reia.hook(use_element, ("span", parent));
    reia.bare_container_node(span.into())
}
