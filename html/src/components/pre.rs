use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlPreElement;
pub fn pre(cc: ComponentBuilder, props: HtmlProps<HtmlPreElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlPreElement>,
        UseElementArgs {
            tag_name: "pre",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
