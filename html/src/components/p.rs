use crate::elem::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlParagraphElement;
pub fn p(
    reia: ComponentBuilder,
    props: ElementProps<HtmlParagraphElement>,
) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlParagraphElement>,
        UseElementArgs {
            tag_name: "p",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
