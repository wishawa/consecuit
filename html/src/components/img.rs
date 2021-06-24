use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlImageElement;
pub fn img(cc: ComponentBuilder, props: HtmlProps<HtmlImageElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlImageElement>,
        UseElementArgs {
            tag_name: "img",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
