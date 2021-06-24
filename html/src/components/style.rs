use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlStyleElement;
pub fn style(cc: ComponentBuilder, props: HtmlProps<HtmlStyleElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlStyleElement>,
        UseElementArgs {
            tag_name: "style",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
