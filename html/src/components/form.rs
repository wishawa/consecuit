use crate::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlFormElement;
pub fn form(reia: ComponentBuilder, props: ElementProps<HtmlFormElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlFormElement>,
        UseElementArgs {
            tag_name: "form",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
