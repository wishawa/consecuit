use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlTableSectionElement;
pub fn thead(
    cc: ComponentBuilder,
    props: HtmlProps<HtmlTableSectionElement>,
) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlTableSectionElement>,
        UseElementArgs {
            tag_name: "thead",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
