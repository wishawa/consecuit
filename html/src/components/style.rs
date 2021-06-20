use crate::elem::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlStyleElement;
pub fn style(
    reia: ComponentBuilder,
    props: ElementProps<HtmlStyleElement>,
) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlStyleElement>,
        UseElementArgs {
            tag_name: "style",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
