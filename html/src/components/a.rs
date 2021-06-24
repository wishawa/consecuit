use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlAnchorElement;
pub fn a(cc: ComponentBuilder, props: HtmlProps<HtmlAnchorElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlAnchorElement>,
        UseElementArgs {
            tag_name: "a",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
