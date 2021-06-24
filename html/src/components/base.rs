use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlBaseElement;
pub fn base(cc: ComponentBuilder, props: HtmlProps<HtmlBaseElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlBaseElement>,
        UseElementArgs {
            tag_name: "base",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
