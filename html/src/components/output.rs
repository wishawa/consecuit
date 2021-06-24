use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlOutputElement;
pub fn output(cc: ComponentBuilder, props: HtmlProps<HtmlOutputElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlOutputElement>,
        UseElementArgs {
            tag_name: "output",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
