use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlDataElement;
pub fn data(cc: ComponentBuilder, props: HtmlProps<HtmlDataElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlDataElement>,
        UseElementArgs {
            tag_name: "data",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
