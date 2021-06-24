use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlMapElement;
pub fn map(cc: ComponentBuilder, props: HtmlProps<HtmlMapElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlMapElement>,
        UseElementArgs {
            tag_name: "map",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
