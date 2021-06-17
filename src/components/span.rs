use web_sys::HtmlSpanElement;

use crate::{hooks::use_element, ComponentBuilder, ContainerReturn};

#[derive(PartialEq, Clone)]
pub struct SpanProps {}
pub fn span(reia: ComponentBuilder, _props: SpanProps) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.parent_node.clone();
    let (reia, span): (_, HtmlSpanElement) = reia.hook(use_element, ("span", parent));
    reia.bare_container_node(span.into())
}
