use crate::elem::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlSpanElement;
pub fn span(reia: ComponentBuilder, props: ElementProps<HtmlSpanElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlSpanElement>,
        UseElementArgs {
            tag_name: "span",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
