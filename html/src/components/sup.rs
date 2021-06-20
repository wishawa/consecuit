use crate::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlElement;
pub fn sup(reia: ComponentBuilder, props: ElementProps<HtmlElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlElement>,
        UseElementArgs {
            tag_name: "sup",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
