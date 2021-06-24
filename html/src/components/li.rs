use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlLiElement;
pub fn li(cc: ComponentBuilder, props: HtmlProps<HtmlLiElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlLiElement>,
        UseElementArgs {
            tag_name: "li",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
