use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlAreaElement;
pub fn area(cc: ComponentBuilder, props: HtmlProps<HtmlAreaElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlAreaElement>,
        UseElementArgs {
            tag_name: "area",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
