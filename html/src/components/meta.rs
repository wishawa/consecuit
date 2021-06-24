use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlMetaElement;
pub fn meta(cc: ComponentBuilder, props: HtmlProps<HtmlMetaElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlMetaElement>,
        UseElementArgs {
            tag_name: "meta",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
