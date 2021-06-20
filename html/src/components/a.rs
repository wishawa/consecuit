use crate::elem::{use_element, HtmlProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlAnchorElement;
pub fn a(reia: ComponentBuilder, props: HtmlProps<HtmlAnchorElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlAnchorElement>,
        UseElementArgs {
            tag_name: "a",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
