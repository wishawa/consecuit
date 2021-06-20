use crate::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlLinkElement;
pub fn link(reia: ComponentBuilder, props: ElementProps<HtmlLinkElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlLinkElement>,
        UseElementArgs {
            tag_name: "link",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
