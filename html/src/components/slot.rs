use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlSlotElement;
pub fn slot(cc: ComponentBuilder, props: HtmlProps<HtmlSlotElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlSlotElement>,
        UseElementArgs {
            tag_name: "slot",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
