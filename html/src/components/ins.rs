use crate::elem::{use_element, HtmlProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlModElement;
pub fn ins(reia: ComponentBuilder, props: HtmlProps<HtmlModElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlModElement>,
        UseElementArgs {
            tag_name: "ins",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
