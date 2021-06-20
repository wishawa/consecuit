use crate::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlTextAreaElement;
pub fn textarea(
    reia: ComponentBuilder,
    props: ElementProps<HtmlTextAreaElement>,
) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlTextAreaElement>,
        UseElementArgs {
            tag_name: "textarea",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
