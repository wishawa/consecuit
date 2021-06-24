use crate::elem::{use_element, HtmlProps, UseElementArgs};
use reia::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlLiElement;
pub fn li(reia: ComponentBuilder, props: HtmlProps<HtmlLiElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlLiElement>,
        UseElementArgs {
            tag_name: "li",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
