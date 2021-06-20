use crate::elem::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlBrElement;
pub fn br(reia: ComponentBuilder, props: ElementProps<HtmlBrElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlBrElement>,
        UseElementArgs {
            tag_name: "br",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
