use crate::elem::{use_element, HtmlProps, UseElementArgs};
use reia::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlTextAreaElement;
pub fn textarea(
    reia: ComponentBuilder,
    props: HtmlProps<HtmlTextAreaElement>,
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
