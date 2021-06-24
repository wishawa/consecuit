use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlHeadingElement;
pub fn h6(cc: ComponentBuilder, props: HtmlProps<HtmlHeadingElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlHeadingElement>,
        UseElementArgs {
            tag_name: "h6",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
