use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlTableCaptionElement;
pub fn caption(
    cc: ComponentBuilder,
    props: HtmlProps<HtmlTableCaptionElement>,
) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlTableCaptionElement>,
        UseElementArgs {
            tag_name: "caption",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
