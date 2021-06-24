use crate::elem::{use_element, HtmlProps, UseElementArgs};
use reia::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlTableRowElement;
pub fn tr(reia: ComponentBuilder, props: HtmlProps<HtmlTableRowElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlTableRowElement>,
        UseElementArgs {
            tag_name: "tr",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
