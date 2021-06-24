use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlParagraphElement;
pub fn p(cc: ComponentBuilder, props: HtmlProps<HtmlParagraphElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlParagraphElement>,
        UseElementArgs {
            tag_name: "p",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
