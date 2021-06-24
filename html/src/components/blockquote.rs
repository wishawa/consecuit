use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlQuoteElement;
pub fn blockquote(
    cc: ComponentBuilder,
    props: HtmlProps<HtmlQuoteElement>,
) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlQuoteElement>,
        UseElementArgs {
            tag_name: "blockquote",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
