use crate::elem::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlAreaElement;
pub fn area(reia: ComponentBuilder, props: ElementProps<HtmlAreaElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlAreaElement>,
        UseElementArgs {
            tag_name: "area",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
