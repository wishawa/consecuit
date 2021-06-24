use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlProgressElement;
pub fn progress(
    cc: ComponentBuilder,
    props: HtmlProps<HtmlProgressElement>,
) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlProgressElement>,
        UseElementArgs {
            tag_name: "progress",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
