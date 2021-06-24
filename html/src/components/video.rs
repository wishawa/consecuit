use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlVideoElement;
pub fn video(cc: ComponentBuilder, props: HtmlProps<HtmlVideoElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlVideoElement>,
        UseElementArgs {
            tag_name: "video",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
