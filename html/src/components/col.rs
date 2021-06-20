use crate::elem::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlTableColElement;
pub fn col(
    reia: ComponentBuilder,
    props: ElementProps<HtmlTableColElement>,
) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlTableColElement>,
        UseElementArgs {
            tag_name: "col",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
