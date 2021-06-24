use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlParamElement;
pub fn param(cc: ComponentBuilder, props: HtmlProps<HtmlParamElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlParamElement>,
        UseElementArgs {
            tag_name: "param",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
