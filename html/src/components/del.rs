use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlModElement;
pub fn del(cc: ComponentBuilder, props: HtmlProps<HtmlModElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlModElement>,
        UseElementArgs {
            tag_name: "del",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
