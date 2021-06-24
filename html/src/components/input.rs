use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlInputElement;
pub fn input(cc: ComponentBuilder, props: HtmlProps<HtmlInputElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlInputElement>,
        UseElementArgs {
            tag_name: "input",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
