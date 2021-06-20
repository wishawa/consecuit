use crate::elem::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlOListElement;
pub fn ol(reia: ComponentBuilder, props: ElementProps<HtmlOListElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlOListElement>,
        UseElementArgs {
            tag_name: "ol",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
