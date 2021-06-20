use crate::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlCanvasElement;
pub fn canvas(
    reia: ComponentBuilder,
    props: ElementProps<HtmlCanvasElement>,
) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlCanvasElement>,
        UseElementArgs {
            tag_name: "canvas",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
