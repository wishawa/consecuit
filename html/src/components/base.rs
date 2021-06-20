use crate::elem::{use_element, HtmlProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlBaseElement;
pub fn base(reia: ComponentBuilder, props: HtmlProps<HtmlBaseElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlBaseElement>,
        UseElementArgs {
            tag_name: "base",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
