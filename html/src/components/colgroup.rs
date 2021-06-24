use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlTableColElement;
pub fn colgroup(
    cc: ComponentBuilder,
    props: HtmlProps<HtmlTableColElement>,
) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlTableColElement>,
        UseElementArgs {
            tag_name: "colgroup",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
