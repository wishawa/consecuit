use crate::elem::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlMapElement;
pub fn map(reia: ComponentBuilder, props: ElementProps<HtmlMapElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlMapElement>,
        UseElementArgs {
            tag_name: "map",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
