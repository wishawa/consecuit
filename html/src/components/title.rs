use crate::elem::{use_element, HtmlProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlTitleElement;
pub fn title(reia: ComponentBuilder, props: HtmlProps<HtmlTitleElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlTitleElement>,
        UseElementArgs {
            tag_name: "title",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
