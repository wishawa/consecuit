use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlMeterElement;
pub fn meter(cc: ComponentBuilder, props: HtmlProps<HtmlMeterElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlMeterElement>,
        UseElementArgs {
            tag_name: "meter",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
