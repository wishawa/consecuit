use crate::elem::{use_element, HtmlProps, UseElementArgs};
use reia::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlLinkElement;
pub fn link(reia: ComponentBuilder, props: HtmlProps<HtmlLinkElement>) -> impl ContainerReturn {
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
