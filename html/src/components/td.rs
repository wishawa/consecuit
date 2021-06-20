use crate::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlTableCellElement;
pub fn td(
    reia: ComponentBuilder,
    props: ElementProps<HtmlTableCellElement>,
) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlTableCellElement>,
        UseElementArgs {
            tag_name: "td",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
