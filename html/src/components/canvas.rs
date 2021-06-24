use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlCanvasElement;
pub fn canvas(cc: ComponentBuilder, props: HtmlProps<HtmlCanvasElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlCanvasElement>,
        UseElementArgs {
            tag_name: "canvas",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
