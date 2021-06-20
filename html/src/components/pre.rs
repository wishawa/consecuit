use crate::elem::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlPreElement;
pub fn pre(reia: ComponentBuilder, props: ElementProps<HtmlPreElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlPreElement>,
        UseElementArgs {
            tag_name: "pre",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
