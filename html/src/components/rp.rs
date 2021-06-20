use crate::elem::{use_element, HtmlProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlElement;
pub fn rp(reia: ComponentBuilder, props: HtmlProps<HtmlElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlElement>,
        UseElementArgs {
            tag_name: "rp",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
