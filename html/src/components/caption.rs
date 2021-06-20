use crate::elem::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlTableCaptionElement;
pub fn caption(
    reia: ComponentBuilder,
    props: ElementProps<HtmlTableCaptionElement>,
) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlTableCaptionElement>,
        UseElementArgs {
            tag_name: "caption",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
