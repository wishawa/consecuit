use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlDetailsElement;
pub fn details(cc: ComponentBuilder, props: HtmlProps<HtmlDetailsElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlDetailsElement>,
        UseElementArgs {
            tag_name: "details",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
