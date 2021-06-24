use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlTitleElement;
pub fn title(cc: ComponentBuilder, props: HtmlProps<HtmlTitleElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlTitleElement>,
        UseElementArgs {
            tag_name: "title",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
