use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlOptGroupElement;
pub fn optgroup(
    cc: ComponentBuilder,
    props: HtmlProps<HtmlOptGroupElement>,
) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlOptGroupElement>,
        UseElementArgs {
            tag_name: "optgroup",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
