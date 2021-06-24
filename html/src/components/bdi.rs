use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlElement;
pub fn bdi(cc: ComponentBuilder, props: HtmlProps<HtmlElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlElement>,
        UseElementArgs {
            tag_name: "bdi",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
