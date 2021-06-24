use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlSourceElement;
pub fn source(cc: ComponentBuilder, props: HtmlProps<HtmlSourceElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlSourceElement>,
        UseElementArgs {
            tag_name: "source",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
