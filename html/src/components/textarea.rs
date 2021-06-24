use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlTextAreaElement;
pub fn textarea(
    cc: ComponentBuilder,
    props: HtmlProps<HtmlTextAreaElement>,
) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlTextAreaElement>,
        UseElementArgs {
            tag_name: "textarea",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
