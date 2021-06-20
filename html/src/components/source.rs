use crate::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlSourceElement;
pub fn source(
    reia: ComponentBuilder,
    props: ElementProps<HtmlSourceElement>,
) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlSourceElement>,
        UseElementArgs {
            tag_name: "source",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
