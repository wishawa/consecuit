use crate::elem::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlButtonElement;
pub fn button(
    reia: ComponentBuilder,
    props: ElementProps<HtmlButtonElement>,
) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlButtonElement>,
        UseElementArgs {
            tag_name: "button",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
