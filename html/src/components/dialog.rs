use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlDialogElement;
pub fn dialog(cc: ComponentBuilder, props: HtmlProps<HtmlDialogElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlDialogElement>,
        UseElementArgs {
            tag_name: "dialog",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
