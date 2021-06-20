use crate::elem::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlTableCellElement;
pub fn th(
    reia: ComponentBuilder,
    props: ElementProps<HtmlTableCellElement>,
) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlTableCellElement>,
        UseElementArgs {
            tag_name: "th",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
