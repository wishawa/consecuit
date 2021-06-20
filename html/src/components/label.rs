use crate::elem::{use_element, HtmlProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlLabelElement;
pub fn label(reia: ComponentBuilder, props: HtmlProps<HtmlLabelElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlLabelElement>,
        UseElementArgs {
            tag_name: "label",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
