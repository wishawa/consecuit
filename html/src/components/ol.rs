use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlOListElement;
pub fn ol(cc: ComponentBuilder, props: HtmlProps<HtmlOListElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlOListElement>,
        UseElementArgs {
            tag_name: "ol",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
