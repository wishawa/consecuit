use crate::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlUListElement;
pub fn ul(reia: ComponentBuilder, props: ElementProps<HtmlUListElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlUListElement>,
        UseElementArgs {
            tag_name: "ul",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
