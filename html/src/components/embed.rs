use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlEmbedElement;
pub fn embed(cc: ComponentBuilder, props: HtmlProps<HtmlEmbedElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlEmbedElement>,
        UseElementArgs {
            tag_name: "embed",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
