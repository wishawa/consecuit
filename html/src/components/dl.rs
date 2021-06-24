use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlDListElement;
pub fn dl(cc: ComponentBuilder, props: HtmlProps<HtmlDListElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlDListElement>,
        UseElementArgs {
            tag_name: "dl",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
