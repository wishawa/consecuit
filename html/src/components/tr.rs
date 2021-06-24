use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlTableRowElement;
pub fn tr(cc: ComponentBuilder, props: HtmlProps<HtmlTableRowElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlTableRowElement>,
        UseElementArgs {
            tag_name: "tr",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
