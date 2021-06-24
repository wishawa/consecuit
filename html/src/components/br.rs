use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlBrElement;
pub fn br(cc: ComponentBuilder, props: HtmlProps<HtmlBrElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlBrElement>,
        UseElementArgs {
            tag_name: "br",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
