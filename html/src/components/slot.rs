use crate::elem::{use_element, HtmlProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlSlotElement;
pub fn slot(reia: ComponentBuilder, props: HtmlProps<HtmlSlotElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlSlotElement>,
        UseElementArgs {
            tag_name: "slot",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
