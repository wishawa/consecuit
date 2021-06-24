use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlUListElement;
pub fn ul(cc: ComponentBuilder, props: HtmlProps<HtmlUListElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlUListElement>,
        UseElementArgs {
            tag_name: "ul",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
