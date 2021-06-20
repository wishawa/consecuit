use crate::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlTableSectionElement;
pub fn thead(
    reia: ComponentBuilder,
    props: ElementProps<HtmlTableSectionElement>,
) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlTableSectionElement>,
        UseElementArgs {
            tag_name: "thead",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
