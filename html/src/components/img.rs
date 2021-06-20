use crate::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlImageElement;
pub fn img(reia: ComponentBuilder, props: ElementProps<HtmlImageElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlImageElement>,
        UseElementArgs {
            tag_name: "img",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
