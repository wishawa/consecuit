use crate::elem::{use_element, HtmlProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlQuoteElement;
pub fn q(reia: ComponentBuilder, props: HtmlProps<HtmlQuoteElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlQuoteElement>,
        UseElementArgs {
            tag_name: "q",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
