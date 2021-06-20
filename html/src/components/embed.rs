use crate::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlEmbedElement;
pub fn embed(
    reia: ComponentBuilder,
    props: ElementProps<HtmlEmbedElement>,
) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlEmbedElement>,
        UseElementArgs {
            tag_name: "embed",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
