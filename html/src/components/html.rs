use crate::elem::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlHtmlElement;
pub fn html(reia: ComponentBuilder, props: ElementProps<HtmlHtmlElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlHtmlElement>,
        UseElementArgs {
            tag_name: "html",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
