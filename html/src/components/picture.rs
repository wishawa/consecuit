use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlPictureElement;
pub fn picture(cc: ComponentBuilder, props: HtmlProps<HtmlPictureElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlPictureElement>,
        UseElementArgs {
            tag_name: "picture",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
