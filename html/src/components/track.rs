use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlTrackElement;
pub fn track(cc: ComponentBuilder, props: HtmlProps<HtmlTrackElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlTrackElement>,
        UseElementArgs {
            tag_name: "track",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
