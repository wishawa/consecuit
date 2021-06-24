use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlSelectElement;
pub fn select(cc: ComponentBuilder, props: HtmlProps<HtmlSelectElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlSelectElement>,
        UseElementArgs {
            tag_name: "select",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
