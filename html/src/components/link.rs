use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlLinkElement;
pub fn link(cc: ComponentBuilder, props: HtmlProps<HtmlLinkElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlLinkElement>,
        UseElementArgs {
            tag_name: "link",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
