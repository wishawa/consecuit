use crate::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlProgressElement;
pub fn progress(
    reia: ComponentBuilder,
    props: ElementProps<HtmlProgressElement>,
) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlProgressElement>,
        UseElementArgs {
            tag_name: "progress",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
