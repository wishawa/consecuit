use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlTableElement;
pub fn table(cc: ComponentBuilder, props: HtmlProps<HtmlTableElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlTableElement>,
        UseElementArgs {
            tag_name: "table",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
