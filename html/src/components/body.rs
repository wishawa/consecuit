use crate::elem::{use_element, HtmlProps, UseElementArgs};
use reia::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlBodyElement;
pub fn body(reia: ComponentBuilder, props: HtmlProps<HtmlBodyElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlBodyElement>,
        UseElementArgs {
            tag_name: "body",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
