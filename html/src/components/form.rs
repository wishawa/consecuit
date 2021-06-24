use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlFormElement;
pub fn form(cc: ComponentBuilder, props: HtmlProps<HtmlFormElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlFormElement>,
        UseElementArgs {
            tag_name: "form",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
