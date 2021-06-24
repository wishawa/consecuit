use crate::elem::{use_element, HtmlProps, UseElementArgs};
use reia::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlElement;
pub fn mark(reia: ComponentBuilder, props: HtmlProps<HtmlElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlElement>,
        UseElementArgs {
            tag_name: "mark",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
