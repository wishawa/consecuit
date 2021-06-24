use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlLabelElement;
pub fn label(cc: ComponentBuilder, props: HtmlProps<HtmlLabelElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlLabelElement>,
        UseElementArgs {
            tag_name: "label",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
