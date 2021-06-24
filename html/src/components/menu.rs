use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlMenuElement;
pub fn menu(cc: ComponentBuilder, props: HtmlProps<HtmlMenuElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlMenuElement>,
        UseElementArgs {
            tag_name: "menu",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
