use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlAudioElement;
pub fn audio(cc: ComponentBuilder, props: HtmlProps<HtmlAudioElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlAudioElement>,
        UseElementArgs {
            tag_name: "audio",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
