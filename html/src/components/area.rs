use crate::elem::{use_element, HtmlProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlAreaElement;
pub fn area(reia: ComponentBuilder, props: HtmlProps<HtmlAreaElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlAreaElement>,
        UseElementArgs {
            tag_name: "area",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
