use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlHtmlElement;
pub fn html(cc: ComponentBuilder, props: HtmlProps<HtmlHtmlElement>) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlHtmlElement>,
        UseElementArgs {
            tag_name: "html",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
