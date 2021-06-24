use crate::elem::{use_element, HtmlProps, UseElementArgs};
use reia::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlDivElement;
pub fn div(reia: ComponentBuilder, props: HtmlProps<HtmlDivElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlDivElement>,
        UseElementArgs {
            tag_name: "div",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
