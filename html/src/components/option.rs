use crate::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlOptionElement;
pub fn option(
    reia: ComponentBuilder,
    props: ElementProps<HtmlOptionElement>,
) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlOptionElement>,
        UseElementArgs {
            tag_name: "option",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
