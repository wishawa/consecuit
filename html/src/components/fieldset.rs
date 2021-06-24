use crate::elem::{use_element, HtmlProps, UseElementArgs};
use consecuit::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlFieldSetElement;
pub fn fieldset(
    cc: ComponentBuilder,
    props: HtmlProps<HtmlFieldSetElement>,
) -> impl ContainerReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, elem) = cc.hook(
        use_element::<HtmlFieldSetElement>,
        UseElementArgs {
            tag_name: "fieldset",
            props,
            parent,
        },
    );
    cc.bare_container_node(elem.into())
}
