use crate::elem::{use_element, HtmlProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlSelectElement;
pub fn select(reia: ComponentBuilder, props: HtmlProps<HtmlSelectElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlSelectElement>,
        UseElementArgs {
            tag_name: "select",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
