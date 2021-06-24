use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlScriptElement;
pub fn script(cc: ComponentBuilder, props: HtmlProps<HtmlScriptElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlScriptElement>,
        UseElementArgs {
            tag_name: "script",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
