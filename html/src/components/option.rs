use crate::elem::{use_element, HtmlProps, UseElementArgs};
use reia::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlOptionElement;
pub fn option(reia: ComponentBuilder, props: HtmlProps<HtmlOptionElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlOptionElement>,
        UseElementArgs {
            tag_name: "option",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
