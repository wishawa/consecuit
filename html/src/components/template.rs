use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlTemplateElement;
pub fn template(
    cc: ComponentBuilder,
    props: HtmlProps<HtmlTemplateElement>,
) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlTemplateElement>,
        UseElementArgs {
            tag_name: "template",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
