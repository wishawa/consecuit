use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlIFrameElement;
pub fn iframe(cc: ComponentBuilder, props: HtmlProps<HtmlIFrameElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlIFrameElement>,
        UseElementArgs {
            tag_name: "iframe",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
