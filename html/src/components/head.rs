use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlHeadElement;
pub fn head(cc: ComponentBuilder, props: HtmlProps<HtmlHeadElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlHeadElement>,
        UseElementArgs {
            tag_name: "head",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
