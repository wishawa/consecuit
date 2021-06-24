use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlOptionElement;
pub fn option(cc: ComponentBuilder, props: HtmlProps<HtmlOptionElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlOptionElement>,
        UseElementArgs {
            tag_name: "option",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
