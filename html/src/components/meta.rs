use crate::elem::{use_element, HtmlProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlMetaElement;
pub fn meta(reia: ComponentBuilder, props: HtmlProps<HtmlMetaElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlMetaElement>,
        UseElementArgs {
            tag_name: "meta",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}