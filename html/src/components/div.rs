use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlDivElement;
pub fn div(cc: ComponentBuilder, props: HtmlProps<HtmlDivElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlDivElement>,
        UseElementArgs {
            tag_name: "div",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
