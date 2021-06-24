use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlTableCellElement;
pub fn th(cc: ComponentBuilder, props: HtmlProps<HtmlTableCellElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlTableCellElement>,
        UseElementArgs {
            tag_name: "th",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
