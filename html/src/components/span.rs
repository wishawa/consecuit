use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlSpanElement;
pub fn span(cc: ComponentBuilder, props: HtmlProps<HtmlSpanElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlSpanElement>,
        UseElementArgs {
            tag_name: "span",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
